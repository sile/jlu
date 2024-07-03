#[derive(Debug, clap::Args)]
pub struct FlattenCommand {}

impl FlattenCommand {
    pub fn run(&self) -> orfail::Result<()> {
        Ok(())
    }
}
