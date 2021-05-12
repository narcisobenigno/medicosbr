use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Debug;

// For more details see [here](https://ibge.gov.br/explica/codigos-dos-municipios.php)
// and [here](https://gitlab.procc.fiocruz.br/mave/repo/-/tree/master/Dados/InfoGripe#alerta-de-situa%C3%A7%C3%A3o-com-base-no-n%C3%BAmero-de-novos-casos-semanais)

#[derive(Debug, PartialEq)]
pub struct ParseRegionError();

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Region {
    Acre,
    Alagoas,
    Amapa,
    Amazonas,
    Bahia,
    Ceara,
    DistritoFederal,
    EspiritoSanto,
    Goias,
    Maranhao,
    MatoGrosso,
    MatoGrossoDoSul,
    MinasGerais,
    Para,
    Paraiba,
    Parana,
    Pernambuco,
    Piaui,
    RioGrandeDoNorte,
    RioGrandeDoSul,
    RioDeJaneiro,
    Rondonia,
    Roraima,
    SantaCatarina,
    SaoPaulo,
    Sergipe,
    Tocantins,
}

impl TryFrom<u16> for Region {
    type Error = ParseRegionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            12 => Ok(Region::Acre),
            27 => Ok(Region::Alagoas),
            16 => Ok(Region::Amapa),
            13 => Ok(Region::Amazonas),
            29 => Ok(Region::Bahia),
            23 => Ok(Region::Ceara),
            53 => Ok(Region::DistritoFederal),
            32 => Ok(Region::EspiritoSanto),
            52 => Ok(Region::Goias),
            21 => Ok(Region::Maranhao),
            51 => Ok(Region::MatoGrosso),
            50 => Ok(Region::MatoGrossoDoSul),
            31 => Ok(Region::MinasGerais),
            15 => Ok(Region::Para),
            25 => Ok(Region::Paraiba),
            41 => Ok(Region::Parana),
            26 => Ok(Region::Pernambuco),
            22 => Ok(Region::Piaui),
            24 => Ok(Region::RioGrandeDoNorte),
            43 => Ok(Region::RioGrandeDoSul),
            33 => Ok(Region::RioDeJaneiro),
            11 => Ok(Region::Rondonia),
            14 => Ok(Region::Roraima),
            42 => Ok(Region::SantaCatarina),
            35 => Ok(Region::SaoPaulo),
            28 => Ok(Region::Sergipe),
            17 => Ok(Region::Tocantins),
            _ => Err(ParseRegionError()),
        }
    }
}

impl Region {
    pub fn name(&self) -> &str {
        match self {
            Region::Acre => "Acre",
            Region::Alagoas => "Alagoas",
            Region::Amapa => "Amapa",
            Region::Amazonas => "Amazonas",
            Region::Bahia => "Bahia",
            Region::Ceara => "Ceara",
            Region::DistritoFederal => "DistritoFederal",
            Region::EspiritoSanto => "EspiritoSanto",
            Region::Goias => "Goias",
            Region::Maranhao => "Maranhao",
            Region::MatoGrosso => "MatoGrosso",
            Region::MatoGrossoDoSul => "MatoGrossoDoSul",
            Region::MinasGerais => "MinasGerais",
            Region::Para => "Para",
            Region::Paraiba => "Paraiba",
            Region::Parana => "Parana",
            Region::Pernambuco => "Pernambuco",
            Region::Piaui => "Piaui",
            Region::RioGrandeDoNorte => "RioGrandeDoNorte",
            Region::RioGrandeDoSul => "RioGrandeDoSul",
            Region::RioDeJaneiro => "RioDeJaneiro",
            Region::Rondonia => "Rondonia",
            Region::Roraima => "Roraima",
            Region::SantaCatarina => "SantaCatarina",
            Region::SaoPaulo => "SaoPaulo",
            Region::Sergipe => "Sergipe",
            Region::Tocantins => "Tocantins",
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::ParseRegionError;
    use super::Region;

    #[test]
    fn it_parse_regions() {
        vec![
            ParseCase {
                expected: Ok(Region::Acre),
                region_code: 12,
            },
            ParseCase {
                expected: Ok(Region::Alagoas),
                region_code: 27,
            },
            ParseCase {
                expected: Ok(Region::Amapa),
                region_code: 16,
            },
            ParseCase {
                expected: Ok(Region::Amazonas),
                region_code: 13,
            },
            ParseCase {
                expected: Ok(Region::Bahia),
                region_code: 29,
            },
            ParseCase {
                expected: Ok(Region::Ceara),
                region_code: 23,
            },
            ParseCase {
                expected: Ok(Region::DistritoFederal),
                region_code: 53,
            },
            ParseCase {
                expected: Ok(Region::EspiritoSanto),
                region_code: 32,
            },
            ParseCase {
                expected: Ok(Region::Goias),
                region_code: 52,
            },
            ParseCase {
                expected: Ok(Region::Maranhao),
                region_code: 21,
            },
            ParseCase {
                expected: Ok(Region::MatoGrosso),
                region_code: 51,
            },
            ParseCase {
                expected: Ok(Region::MatoGrossoDoSul),
                region_code: 50,
            },
            ParseCase {
                expected: Ok(Region::MinasGerais),
                region_code: 31,
            },
            ParseCase {
                expected: Ok(Region::Para),
                region_code: 15,
            },
            ParseCase {
                expected: Ok(Region::Paraiba),
                region_code: 25,
            },
            ParseCase {
                expected: Ok(Region::Parana),
                region_code: 41,
            },
            ParseCase {
                expected: Ok(Region::Pernambuco),
                region_code: 26,
            },
            ParseCase {
                expected: Ok(Region::Piaui),
                region_code: 22,
            },
            ParseCase {
                expected: Ok(Region::RioGrandeDoNorte),
                region_code: 24,
            },
            ParseCase {
                expected: Ok(Region::RioGrandeDoSul),
                region_code: 43,
            },
            ParseCase {
                expected: Ok(Region::RioDeJaneiro),
                region_code: 33,
            },
            ParseCase {
                expected: Ok(Region::Rondonia),
                region_code: 11,
            },
            ParseCase {
                expected: Ok(Region::Roraima),
                region_code: 14,
            },
            ParseCase {
                expected: Ok(Region::SantaCatarina),
                region_code: 42,
            },
            ParseCase {
                expected: Ok(Region::SaoPaulo),
                region_code: 35,
            },
            ParseCase {
                expected: Ok(Region::Sergipe),
                region_code: 28,
            },
            ParseCase {
                expected: Ok(Region::Tocantins),
                region_code: 17,
            },
            ParseCase {
                expected: Err(ParseRegionError()),
                region_code: 33333,
            },
        ]
        .iter()
        .for_each(move |case| {
            assert_eq!(case.expected, Region::try_from(case.region_code.clone()))
        });
    }

    #[test]
    fn it_returns_name() {
        vec![
            NameCase {
                expected: "Acre".to_string(),
                region: Region::Acre,
            },
            NameCase {
                expected: "Alagoas".to_string(),
                region: Region::Alagoas,
            },
            NameCase {
                expected: "Amapa".to_string(),
                region: Region::Amapa,
            },
            NameCase {
                expected: "Amazonas".to_string(),
                region: Region::Amazonas,
            },
            NameCase {
                expected: "Bahia".to_string(),
                region: Region::Bahia,
            },
            NameCase {
                expected: "Ceara".to_string(),
                region: Region::Ceara,
            },
            NameCase {
                expected: "DistritoFederal".to_string(),
                region: Region::DistritoFederal,
            },
            NameCase {
                expected: "EspiritoSanto".to_string(),
                region: Region::EspiritoSanto,
            },
            NameCase {
                expected: "Goias".to_string(),
                region: Region::Goias,
            },
            NameCase {
                expected: "Maranhao".to_string(),
                region: Region::Maranhao,
            },
            NameCase {
                expected: "MatoGrosso".to_string(),
                region: Region::MatoGrosso,
            },
            NameCase {
                expected: "MatoGrossoDoSul".to_string(),
                region: Region::MatoGrossoDoSul,
            },
            NameCase {
                expected: "MinasGerais".to_string(),
                region: Region::MinasGerais,
            },
            NameCase {
                expected: "Para".to_string(),
                region: Region::Para,
            },
            NameCase {
                expected: "Paraiba".to_string(),
                region: Region::Paraiba,
            },
            NameCase {
                expected: "Parana".to_string(),
                region: Region::Parana,
            },
            NameCase {
                expected: "Pernambuco".to_string(),
                region: Region::Pernambuco,
            },
            NameCase {
                expected: "Piaui".to_string(),
                region: Region::Piaui,
            },
            NameCase {
                expected: "RioGrandeDoNorte".to_string(),
                region: Region::RioGrandeDoNorte,
            },
            NameCase {
                expected: "RioGrandeDoSul".to_string(),
                region: Region::RioGrandeDoSul,
            },
            NameCase {
                expected: "RioDeJaneiro".to_string(),
                region: Region::RioDeJaneiro,
            },
            NameCase {
                expected: "Rondonia".to_string(),
                region: Region::Rondonia,
            },
            NameCase {
                expected: "Roraima".to_string(),
                region: Region::Roraima,
            },
            NameCase {
                expected: "SantaCatarina".to_string(),
                region: Region::SantaCatarina,
            },
            NameCase {
                expected: "SaoPaulo".to_string(),
                region: Region::SaoPaulo,
            },
            NameCase {
                expected: "Sergipe".to_string(),
                region: Region::Sergipe,
            },
            NameCase {
                expected: "Tocantins".to_string(),
                region: Region::Tocantins,
            },
        ]
        .iter()
        .for_each(move |case| assert_eq!(case.expected, case.region.name()));
    }

    struct ParseCase {
        expected: Result<Region, ParseRegionError>,
        region_code: u16,
    }
    struct NameCase {
        expected: String,
        region: Region,
    }
}
