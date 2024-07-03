use clap::Parser;
use jlu::{
    command_count::CountCommand, command_flatten::FlattenCommand, command_rename::RenameCommand,
    command_table::TableCommand,
};
use orfail::OrFail;

#[derive(Parser)]
enum Args {
    Count(CountCommand),
    Flatten(FlattenCommand),
    Rename(RenameCommand),
    Table(TableCommand),
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::Count(c) => c.run().or_fail()?,
        Args::Flatten(c) => c.run().or_fail()?,
        Args::Rename(c) => c.run().or_fail()?,
        Args::Table(c) => c.run().or_fail()?,
    }
    Ok(())
}
