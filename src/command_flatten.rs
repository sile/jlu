use crate::jsonl;
use orfail::OrFail;
use serde_json::{Map, Value};

#[derive(Debug, clap::Args)]
pub struct FlattenCommand {}

impl FlattenCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let output_values = jsonl::values_from_stdin().map(|input_value| {
            let input_value = input_value.or_fail()?;
            let mut flattened = Map::new();
            self.flatten(input_value, "", &mut flattened);
            Ok(Value::Object(flattened))
        });
        jsonl::values_to_stdout(output_values).or_fail()?;
        Ok(())
    }

    fn flatten(&self, value: Value, prefix: &str, output: &mut Map<String, Value>) {
        match value {
            Value::Null => {
                output.insert(prefix.to_owned(), Value::Null);
            }
            Value::Bool(v) => {
                output.insert(prefix.to_owned(), Value::Bool(v));
            }
            Value::Number(v) => {
                output.insert(prefix.to_owned(), Value::Number(v));
            }
            Value::String(v) => {
                output.insert(prefix.to_owned(), Value::String(v));
            }
            Value::Array(array) => {
                let width = array.len().saturating_sub(1).to_string().len();
                for (i, v) in array.into_iter().enumerate() {
                    let key = format!("{prefix}[{i:0width$}]");
                    self.flatten(v, &key, output);
                }
            }
            Value::Object(object) => {
                for (k, v) in object {
                    let key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{prefix}.{k}")
                    };
                    self.flatten(v, &key, output);
                }
            }
        }
    }
}
