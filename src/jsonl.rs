use orfail::OrFail;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;

pub type Object = serde_json::Map<String, Value>;

pub fn from_stdin<T>() -> impl Iterator<Item = orfail::Result<T>>
where
    T: for<'a> Deserialize<'a>,
{
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    serde_json::Deserializer::from_reader(stdin)
        .into_iter()
        .map(|result| result.or_fail())
}

pub fn to_stdout(
    values: impl Iterator<Item = orfail::Result<impl Serialize>>,
) -> orfail::Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for value in values {
        let value = value.or_fail()?;
        let json = serde_json::to_string(&value).or_fail()?;
        if ignore_broken_pipe(writeln!(&mut stdout, "{}", json)).or_fail()? {
            break;
        }
    }
    Ok(())
}

fn ignore_broken_pipe(result: std::io::Result<()>) -> std::io::Result<bool> {
    match result {
        Ok(()) => Ok(false),
        Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => Ok(true),
        Err(err) => Err(err),
    }
}
