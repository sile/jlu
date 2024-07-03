use crate::jsonl;

#[derive(Debug, clap::Args)]
pub struct TableCommand {}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for result in jsonl::values_from_stdin() {
            let _value = result?;
        }
        Ok(())
    }
}
