use pest_01::*;
use pest::Parser;
use anyhow::anyhow;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() -> anyhow::Result<()> {

        let parsed_data = Grammar::parse(Rule::file, "-123.47,-23\n123,132.5,-99\n")?.next().ok_or_else( || anyhow! ("no needed pair"))?;
        let parsed_result = Grammar::parse(Rule::file, "*");

        assert!(parsed_result.is_err());

        assert_eq!(parsed_data.as_str(), "-123.47,-23\n123,132.5,-99\n");
        assert_eq!(parsed_data.as_span().start(), 0);
        
        Ok(())
    }
}
