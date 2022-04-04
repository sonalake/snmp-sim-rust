use core::convert::TryInto;

pub trait GenericTryInto {
    fn try_into_gen<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }
}
