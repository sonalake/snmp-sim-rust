use core::{
    future::Future,
    hint,
    pin::Pin,
    task::{Context as StdContext, Poll},
    time::Duration,
};

use super::actor::Actor;
use super::error::ActixAsyncError;
use super::message::ActorMessage;
use super::runtime::RuntimeService;
use super::util::{
    channel::{OneshotReceiver, SendFuture},
    futures::LocalBoxFuture,
};

/// Message request to actor with timeout setting.
pub type MessageRequest<'a, A, R> = _MessageRequest<<A as Actor>::Runtime, SendFuture<'a, ActorMessage<A>>, R>;

/// Box version of MessageRequest that bound to `Message::Result` type.
pub type BoxedMessageRequest<'a, RT, R> = _MessageRequest<RT, LocalBoxFuture<'a, Result<(), ActixAsyncError>>, R>;

pin_project_lite::pin_project! {
    #[doc(hidden)]
    #[project = MessageRequestProj]
    #[project_replace = MessageRequestReplaceProj]
    pub enum _MessageRequest<RT, Fut, R>
    where
        RT: RuntimeService,
    {
        Request {
            #[pin]
            fut: Fut,
            rx: OneshotReceiver<R>,
            #[pin]
            timeout: Option<RT::Sleep>,
            timeout_response: Option<Duration>
        },
        Response {
            rx: OneshotReceiver<R>,
            #[pin]
            timeout_response: Option<RT::Sleep>
        },
        PlaceHolder,
    }
}

const TIMEOUT_CONFIGURABLE: &str = "Timeout is not configurable after Request Future is polled";

impl<RT, Fut, R> _MessageRequest<RT, Fut, R>
where
    RT: RuntimeService,
{
    pub(crate) fn new(fut: Fut, rx: OneshotReceiver<R>) -> Self {
        _MessageRequest::Request {
            fut,
            rx,
            timeout: None,
            timeout_response: None,
        }
    }

    /// set the timeout duration for request.
    ///
    /// Default to no timeout.
    pub fn timeout(self, dur: Duration) -> Self {
        match self {
            _MessageRequest::Request {
                fut,
                rx,
                timeout_response,
                ..
            } => _MessageRequest::Request {
                fut,
                rx,
                timeout: Some(RT::sleep(dur)),
                timeout_response,
            },
            _ => unreachable!(TIMEOUT_CONFIGURABLE),
        }
    }

    /// set the timeout duration for response.(start from the message arrives at actor)
    ///
    /// Default to no timeout.
    pub fn timeout_response(self, dur: Duration) -> Self {
        match self {
            _MessageRequest::Request { fut, rx, timeout, .. } => _MessageRequest::Request {
                fut,
                rx,
                timeout,
                timeout_response: Some(dur),
            },
            _ => unreachable!(TIMEOUT_CONFIGURABLE),
        }
    }
}

impl<RT, Fut, R> Future for _MessageRequest<RT, Fut, R>
where
    RT: RuntimeService,
    Fut: Future<Output = Result<(), ActixAsyncError>>,
{
    type Output = Result<R, ActixAsyncError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut StdContext<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                MessageRequestProj::Request { fut, timeout, .. } => match fut.poll(cx)? {
                    Poll::Ready(()) => {
                        match self.as_mut().project_replace(_MessageRequest::PlaceHolder) {
                            MessageRequestReplaceProj::Request {
                                rx, timeout_response, ..
                            } => {
                                let timeout_response = timeout_response.map(RT::sleep);
                                self.set(_MessageRequest::Response { rx, timeout_response });
                            }
                            // SAFETY:
                            //
                            // Replace always return the current variant of an enum
                            // Which is Request in this case.
                            _ => unsafe { hint::unreachable_unchecked() },
                        }
                    }
                    Poll::Pending => {
                        return match timeout.as_pin_mut() {
                            Some(timeout) => timeout.poll(cx).map(|_| Err(ActixAsyncError::SendTimeout)),
                            None => Poll::Pending,
                        }
                    }
                },
                MessageRequestProj::Response { rx, timeout_response } => {
                    return match Pin::new(rx).poll(cx) {
                        Poll::Ready(res) => Poll::Ready(Ok(res?)),
                        Poll::Pending => match timeout_response.as_pin_mut() {
                            Some(timeout) => timeout
                                .poll(cx)
                                .map(|_| Err(ActixAsyncError::ReceiveTimeout)),
                            None => Poll::Pending,
                        },
                    }
                }
                MessageRequestProj::PlaceHolder => unreachable!(),
            }
        }
    }
}
