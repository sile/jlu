use crate::jsonl::{self, Object};
use orfail::OrFail;
use regex::Regex;

#[derive(Debug, clap::Args)]
pub struct RenameCommand {
    regex: Regex,
    replacement: String,
}

impl RenameCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let outputs = jsonl::from_stdin::<Object>().map(|input| {
            input.map(|object| {
                object
                    .into_iter()
                    .map(|(key, value)| {
                        let key = self.regex.replace_all(&key, &self.replacement).to_string();
                        (key, value)
                    })
                    .collect::<Object>()
            })
        });
        jsonl::to_stdout(outputs).or_fail()?;
        Ok(())
    }
}
