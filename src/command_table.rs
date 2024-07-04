use crate::jsonl::{self, Object};
use orfail::OrFail;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, clap::Args)]
pub struct TableCommand {
    #[clap(long, short, default_value_t = 50)]
    pub max_column_chars: usize,
    pub column_keys: Vec<String>,
}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut columns = self
            .column_keys
            .iter()
            .map(|key| Column::new(key))
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        for result in jsonl::from_stdin::<Object>() {
            let object = result.or_fail()?;
            let mut row = BTreeMap::new();
            for column in &mut columns {
                let mut value = json_value_to_string(object.get(&column.key));
                if value.chars().count() > self.max_column_chars {
                    let (n, _) = value.char_indices().nth(self.max_column_chars).or_fail()?;
                    value.truncate(n);
                    value.push_str("...");
                }

                column.update_width(&value);
                row.insert(column.key.clone(), value);
            }
            rows.push(row);
        }

        rows.sort_by(|x, y| {
            let xs = columns.iter().map(|c| x.get(&c.key));
            let ys = columns.iter().map(|c| y.get(&c.key));
            xs.cmp(ys)
        });

        for col in &columns {
            print!("| {:<width$} ", col.key, width = col.width);
        }
        println!("|");

        for col in &columns {
            print!("|-{:-<width$}-", "-", width = col.width);
        }
        println!("|");

        let null = "".to_string();
        for row in rows {
            for col in &columns {
                let value = row.get(&col.key).unwrap_or(&null);
                print!("| {:<width$} ", value, width = col.width);
            }
            println!("|");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Column {
    key: String,
    width: usize,
}

impl Column {
    fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            width: key.len(),
        }
    }

    fn update_width(&mut self, value: &str) {
        self.width = self.width.max(value.len());
    }
}

fn json_value_to_string(v: Option<&Value>) -> String {
    let Some(v) = v else {
        return "".to_string();
    };
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => v.replace('|', "\\|"),
        Value::Array(_) => "_ARRAY_".to_string(),
        Value::Object(_) => "_OBJECT_".to_string(),
    }
}
