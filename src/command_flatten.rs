use crate::jsonl;
use orfail::OrFail;
use serde_json::Value;

#[derive(Debug, clap::Args)]
pub struct FlattenCommand {}

impl FlattenCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for result in jsonl::read_from_stdin() {
            let value = result?;
            let flattened_values = flatten(value);
            jsonl::write_to_stdout(flattened_values).or_fail()?;
        }
        Ok(())
    }
}

fn flatten(value: Value) -> impl Iterator<Item = Value> {
    vec![value].into_iter()
}
