use orfail::OrFail;
use std::io::Write;

pub fn read_from_stdin<T>() -> impl Iterator<Item = orfail::Result<T>>
where
    T: serde::de::DeserializeOwned,
{
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    serde_json::Deserializer::from_reader(stdin)
        .into_iter::<T>()
        .map(|result| result.or_fail())
}

pub fn write_to_stdout<T, I>(items: I) -> orfail::Result<()>
where
    T: serde::Serialize,
    I: Iterator<Item = T>,
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for item in items {
        let json = serde_json::to_string(&item).or_fail()?;
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
