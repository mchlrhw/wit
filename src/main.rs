use wit::Parser;

fn main() -> anyhow::Result<()> {
    let source = "1.2 - 34 * 5 - 6.7 / 89";
    let expr = Parser::new(source).parse()?;
    println!("{expr:#?}");

    Ok(())
}
