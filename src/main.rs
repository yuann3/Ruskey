use ruskey::repl::Repl;
use std::io::{self};

fn main() -> io::Result<()> {
    println!("Ruskey Console");

    let mut repl = Repl::new();
    let mut stdin = io::stdin().lock();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    repl.start(&mut stdin, &mut handle)?;

    Ok(())
}
