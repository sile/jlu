use crate::jsonl;

#[derive(Debug, clap::Args)]
pub struct RenameCommand {}

impl RenameCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for result in jsonl::read_from_stdin() {
            let _value = result?;
        }
        Ok(())
    }
}
