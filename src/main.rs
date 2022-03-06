use wit::{codegen::jit_compile, Parser};

fn main() -> anyhow::Result<()> {
    let source = "3 - 2 - 1";
    let expr = Parser::new(source).parse()?;
    let val = jit_compile(expr)?;

    println!("{val}");

    Ok(())
}
