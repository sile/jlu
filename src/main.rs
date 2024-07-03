use clap::Parser;
use jlu::command_flatten::FlattenCommand;
use orfail::OrFail;

#[derive(Parser)]
enum Args {
    Flatten(FlattenCommand),
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::Flatten(c) => c.run().or_fail()?,
    }
    Ok(())
}
