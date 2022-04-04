#[macro_export]
macro_rules! saga_app_data {
    ($saga:ty, $delegate:ty) => {{
        use actix_async::prelude::Actor;
        Data::new(Lazy::<$delegate>::new(|| <$delegate>::new(<$saga>::default().start())))
    }};
}
