use crate::jsonl::{self, Object};
use orfail::OrFail;
use std::collections::BTreeSet;

/// Read JSON objects from stdin and output the unique member names for all top-level objects.
#[derive(Debug, clap::Args)]
pub struct NamesCommand {}

impl NamesCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut outputs = BTreeSet::new();
        for input in jsonl::from_stdin::<Object>() {
            let object = input.or_fail()?;
            outputs.extend(object.keys().cloned());
        }
        jsonl::to_stdout(outputs.into_iter().map(Ok)).or_fail()?;
        Ok(())
    }
}
