use std::collections::BTreeMap;

use crate::jsonl::{self, Object};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct NamesCommand {}

impl NamesCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut output = BTreeMap::new();
        for input in jsonl::from_stdin::<Object>() {
            let object = input.or_fail()?;
            for name in object.keys().cloned() {
                *output.entry(name).or_insert(0) += 1;
            }
        }
        jsonl::to_stdout(std::iter::once(Ok(output))).or_fail()?;
        Ok(())
    }
}
