#[macro_export]
macro_rules! security_scopes {
    ($type:ident : $parent:literal, $($scope:literal),*) => {
        #[derive(Apiv2Security, Deserialize)]
        #[openapi(parent = $parent, scopes($($scope),*))]
        pub struct $type;

        impl actix_web::FromRequest for $type {
            type Error = actix_web::Error;
            type Future = futures::future::Ready<Result<Self, Self::Error>>;

            fn from_request(_: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                futures::future::ready(Ok(Self {}))
            }
        }
    };
}
