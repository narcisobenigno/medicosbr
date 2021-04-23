use std::fmt::Debug;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct ParseCaseError {}

#[derive(PartialEq, Debug)]
pub enum Case {
    SARS,
    SARSCovid17,
    SARSInfluenza,
    SARSDeceased,
    SARSDeceasedInfluenza,
    SARSDeceasedCovid17,
}

impl FromStr for Case {
    type Err = ParseCaseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "srag" => Ok(Case::SARS),
            "sragflu" => Ok(Case::SARSInfluenza),
            "sragcovid" => Ok(Case::SARSCovid17),
            "obito" => Ok(Case::SARSDeceased),
            "obitoflu" => Ok(Case::SARSDeceasedInfluenza),
            "obitocovid" => Ok(Case::SARSDeceasedCovid17),
            _ => Err(ParseCaseError {}),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_case_strings() {
        vec![
            ParseTest::Scenario(Ok(Case::SARS), "srag"),
            ParseTest::Scenario(Ok(Case::SARSCovid17), "sragcovid"),
            ParseTest::Scenario(Ok(Case::SARSInfluenza), "sragflu"),
            ParseTest::Scenario(Ok(Case::SARSDeceased), "obito"),
            ParseTest::Scenario(Ok(Case::SARSDeceasedInfluenza), "obitoflu"),
            ParseTest::Scenario(Ok(Case::SARSDeceasedCovid17), "obitocovid"),
        ]
        .iter()
        .for_each(move |scenario| match scenario {
            ParseTest::Scenario(expected, input) => assert_eq!(expected, &Case::from_str(input)),
        });

        assert_eq!(Err(ParseCaseError {}), Case::from_str("INVALID"))
    }

    enum ParseTest {
        Scenario(Result<Case, ParseCaseError>, &'static str),
    }
}
