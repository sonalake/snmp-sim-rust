use actix_web::FromRequest;
use actix_web::HttpRequest;
use futures::future::{FutureExt, Map};
use paperclip::actix::OperationModifier;
use paperclip::v2::schema::Apiv2Schema as Apiv2SchemaTrait;
use std::ops::{Deref, DerefMut};

pub struct NoSchema<T> {
    t: T,
}

impl<T> NoSchema<T> {
    pub fn into_inner(self) -> T {
        self.t
    }
}

impl<T> From<T> for NoSchema<T> {
    fn from(t: T) -> Self {
        Self { t }
    }
}

impl<T> Deref for NoSchema<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for NoSchema<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T: Default> Default for NoSchema<T> {
    fn default() -> NoSchema<T> {
        NoSchema {
            t: Default::default(),
        }
    }
}

type NoSchemaFromFuture<T> =
    fn(Result<T, <T as FromRequest>::Error>) -> Result<NoSchema<T>, <T as FromRequest>::Error>;

impl<T: FromRequest> FromRequest for NoSchema<T> {
    type Error = T::Error;
    type Future = Map<T::Future, NoSchemaFromFuture<T>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        T::from_request(req, payload).map(|r| r.map(NoSchema::from))
    }
}

impl<T> Apiv2SchemaTrait for NoSchema<T> {}
impl<T> OperationModifier for NoSchema<T> {}
