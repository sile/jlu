use crate::jsonl::{self, Object};
use orfail::OrFail;
use regex::Regex;

/// Read JSON objects from stdin and rename top-level member names that match a regular expression with a replacement string.
///
/// For details about regular expressions and replacement strings,
/// please refer to the documentation of the regex crate: https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace_all
#[derive(Debug, clap::Args)]
pub struct RenameCommand {
    /// Regular expression to match top-level member names.
    regex: Regex,

    /// String to replace the matched segment of the member names.
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
