use crate::jsonl;

#[derive(Debug, clap::Args)]
pub struct CountCommand {
    pub keys: Vec<String>,
}

impl CountCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for result in jsonl::values_from_stdin() {
            let _value = result?;
        }
        Ok(())
    }
}

// #[derive(Debug, serde::Serialize)]
// #[serde(untagged)]
// enum Counter {
//     Value(usize),
//     Children(BTreeMap<String, Self>),
// }

// impl Counter {
//     fn new() -> Self {
//         Self::Value(0)
//     }

//     fn increment<'a>(
//         &mut self,
//         fields: &'a mut impl Iterator<Item = &'a String>,
//         message: &Message,
//     ) {
//         let Some(field) = fields.next() else {
//             let Self::Value(count) = self else {
//                 unreachable!();
//             };
//             *count += 1;
//             return;
//         };

//         let key = message
//             .get_value_string(field)
//             .unwrap_or_else(|| "_OTHER_".to_string());

//         if let Self::Value(_) = self {
//             *self = Self::Children(BTreeMap::new());
//         }
//         let Self::Children(children) = self else {
//             unreachable!();
//         };
//         children
//             .entry(key.to_string())
//             .or_insert_with(Self::new)
//             .increment(fields, message);
//     }
// }
