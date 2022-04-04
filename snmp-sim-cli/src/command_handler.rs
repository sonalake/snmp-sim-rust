use anyhow::Error;

pub trait CommandHandler {
    fn handle(self) -> Result<(), Error>;
}
