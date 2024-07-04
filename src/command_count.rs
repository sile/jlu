use crate::jsonl::{self, Object};
use orfail::OrFail;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Write;

/// Read JSON objects from stdin and count the occurrences of the values associated with the specified top-level member names.
#[derive(Debug, clap::Args)]
pub struct CountCommand {
    /// Names of the top-level members to count.
    pub names: Vec<String>,
}

impl CountCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut counter = Counter::new();
        for result in jsonl::from_stdin::<Object>() {
            let value = Value::Object(result?);
            counter.increment(&mut self.names.iter(), &value);
        }

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        serde_json::to_writer_pretty(&mut stdout, &counter).or_fail()?;
        writeln!(&mut stdout).or_fail()?;

        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
enum Counter {
    Value(usize),
    Children(BTreeMap<String, Self>),
}

impl Counter {
    fn new() -> Self {
        Self::Value(0)
    }

    fn increment<'a>(&mut self, names: &'a mut impl Iterator<Item = &'a String>, value: &Value) {
        let Some(name) = names.next() else {
            let Self::Value(count) = self else {
                unreachable!();
            };
            *count += 1;
            return;
        };

        let key = value
            .as_object()
            .and_then(|object| object.get(name))
            .and_then(|key| match key {
                Value::Null => Some("null".to_string()),
                Value::Bool(v) => Some(v.to_string()),
                Value::Number(v) => Some(v.to_string()),
                Value::String(v) => Some(v.clone()),
                Value::Array(_) => None,
                Value::Object(_) => None,
            })
            .unwrap_or_else(|| "_OTHER_".to_string());

        if let Self::Value(_) = self {
            *self = Self::Children(BTreeMap::new());
        }
        let Self::Children(children) = self else {
            unreachable!();
        };
        children
            .entry(key)
            .or_insert_with(Self::new)
            .increment(names, value);
    }
}
