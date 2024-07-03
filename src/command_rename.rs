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
        let output_values = jsonl::values_from_stdin().map(|input_value| {
            let mut value = input_value.or_fail()?;
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
            Ok(value)
        });
        jsonl::values_to_stdout(output_values).or_fail()?;
        Ok(())
    }
}
