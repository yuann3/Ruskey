use ruskey::repl::Repl;
use std::io::{self};

fn main() -> io::Result<()> {
    //println!("Ruskey Console - AST Parser Mode");
    //println!("Type in commands to see their AST representation");

    let mut repl = Repl::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();

    repl.start(&mut handle, &mut stdout)?;

    Ok(())
}
