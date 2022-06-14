#[cfg(feature = "tokio-rt")]
#[macro_export]
macro_rules! actor {
    ($ty: ty) => {
        impl actix_async::prelude::Actor for $ty {
            type Runtime = actix_async::prelude::TokioRuntime;
        }
    };
}

#[macro_export]
macro_rules! message {
    ($ty: ty, $res: ty) => {
        impl actix_async::prelude::Message for $ty {
            type Result = $res;
        }
    };
}
