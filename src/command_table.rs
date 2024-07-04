use crate::jsonl::{self, Object};
use orfail::OrFail;
use serde_json::Value;
use std::collections::BTreeMap;

/// Read JSON objects from stdin and create a markdown table.
#[derive(Debug, clap::Args)]
pub struct TableCommand {
    /// Names of object members to be included in the table.
    pub column_names: Vec<String>,

    /// If specified, the table rows are sorted based on the member value associated with this name.
    #[clap(long, short)]
    pub sort: Option<String>,

    /// Maximum number of characters to display in a column.
    #[clap(long, short, default_value_t = 50)]
    pub max_column_chars: usize,
}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut columns = self
            .column_names
            .iter()
            .map(|name| Column::new(name))
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        let mut sort_keys = Vec::new();
        for (i, result) in jsonl::from_stdin::<Object>().enumerate() {
            let object = result.or_fail()?;
            let mut row = BTreeMap::new();
            for column in &mut columns {
                let mut value = json_value_to_string(object.get(&column.name));
                if value.chars().count() > self.max_column_chars {
                    let (n, _) = value.char_indices().nth(self.max_column_chars).or_fail()?;
                    value.truncate(n);
                    value.push_str("...");
                }

                column.update_width(&value);
                row.insert(column.name.clone(), value);
            }
            if let Some(name) = &self.sort {
                let value = json_value_to_string(object.get(name));
                sort_keys.push(value);
            }
            rows.push((i, row));
        }

        if self.sort.is_some() {
            rows.sort_by_key(|(i, _)| &sort_keys[*i]);
        }

        for col in &columns {
            print!("| {:<width$} ", col.name, width = col.width);
        }
        println!("|");

        for col in &columns {
            print!("|-{:-<width$}-", "-", width = col.width);
        }
        println!("|");

        let null = "".to_string();
        for (_, row) in rows {
            for col in &columns {
                let value = row.get(&col.name).unwrap_or(&null);
                print!("| {:<width$} ", value, width = col.width);
            }
            println!("|");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    width: usize,
}

impl Column {
    fn new(key: &str) -> Self {
        Self {
            name: key.to_owned(),
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
