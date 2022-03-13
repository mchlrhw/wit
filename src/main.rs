use std::{env, io::Write, process};
use wit::Parser;

fn run(source: &str) -> anyhow::Result<()> {
    let expr = Parser::new(source).parse()?;

    println!("{expr:#?}");

    Ok(())
}

fn run_repl() -> anyhow::Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }

        if let Err(error) = run(&line) {
            println!("Error: {error}");
        }
    }

    Ok(())
}

fn run_file(_path: &str) -> anyhow::Result<()> {
    todo!("running programs from a file is not yet implemented")
}

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match args.len() {
        0 => run_repl(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: wit [script]");
            process::exit(1);
        }
    }
}
