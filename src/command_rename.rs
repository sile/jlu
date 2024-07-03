use crate::jsonl;
use orfail::OrFail;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, clap::Args)]
pub struct RenameCommand {
    regex: Regex,
    replacement: String,
}

impl RenameCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for result in jsonl::read_from_stdin() {
            let mut value = result?;
            if let Value::Object(object) = value {
                let object = object
                    .into_iter()
                    .map(|(key, value)| {
                        let key = self.regex.replace_all(&key, &self.replacement).to_string();
                        (key, value)
                    })
                    .collect();
                value = Value::Object(object);
            }
            jsonl::write_to_stdout(std::iter::once(value)).or_fail()?;
        }
        Ok(())
    }
}
