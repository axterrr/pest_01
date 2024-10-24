use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()>{
    let got = Grammar::parse(Rule::file, "-123.47,-23\n123,132.5,-99\n")?;
    print!("{:?}", got);
    Ok(())
}
