use std::fmt::Debug;

// For more details see [here](https://ibge.gov.br/explica/codigos-dos-municipios.php)
// and [here](https://gitlab.procc.fiocruz.br/mave/repo/-/tree/master/Dados/InfoGripe#alerta-de-situa%C3%A7%C3%A3o-com-base-no-n%C3%BAmero-de-novos-casos-semanais)

#[derive(Debug, PartialEq)]
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

impl Region {
    pub(crate) fn parse(code: &u16) -> Option<Region> {
        match code {
            12 => Some(Region::Acre),
            27 => Some(Region::Alagoas),
            16 => Some(Region::Amapa),
            13 => Some(Region::Amazonas),
            29 => Some(Region::Bahia),
            23 => Some(Region::Ceara),
            53 => Some(Region::DistritoFederal),
            32 => Some(Region::EspiritoSanto),
            52 => Some(Region::Goias),
            21 => Some(Region::Maranhao),
            51 => Some(Region::MatoGrosso),
            50 => Some(Region::MatoGrossoDoSul),
            31 => Some(Region::MinasGerais),
            15 => Some(Region::Para),
            25 => Some(Region::Paraiba),
            41 => Some(Region::Parana),
            26 => Some(Region::Pernambuco),
            22 => Some(Region::Piaui),
            24 => Some(Region::RioGrandeDoNorte),
            43 => Some(Region::RioGrandeDoSul),
            33 => Some(Region::RioDeJaneiro),
            11 => Some(Region::Rondonia),
            14 => Some(Region::Roraima),
            42 => Some(Region::SantaCatarina),
            35 => Some(Region::SaoPaulo),
            28 => Some(Region::Sergipe),
            17 => Some(Region::Tocantins),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Region;

    #[test]
    fn it_parse_regions() {
        vec![
            Test::Case(Some(Region::Acre), 12),
            Test::Case(Some(Region::Alagoas), 27),
            Test::Case(Some(Region::Amapa), 16),
            Test::Case(Some(Region::Amazonas), 13),
            Test::Case(Some(Region::Bahia), 29),
            Test::Case(Some(Region::Ceara), 23),
            Test::Case(Some(Region::DistritoFederal), 53),
            Test::Case(Some(Region::EspiritoSanto), 32),
            Test::Case(Some(Region::Goias), 52),
            Test::Case(Some(Region::Maranhao), 21),
            Test::Case(Some(Region::MatoGrosso), 51),
            Test::Case(Some(Region::MatoGrossoDoSul), 50),
            Test::Case(Some(Region::MinasGerais), 31),
            Test::Case(Some(Region::Para), 15),
            Test::Case(Some(Region::Paraiba), 25),
            Test::Case(Some(Region::Parana), 41),
            Test::Case(Some(Region::Pernambuco), 26),
            Test::Case(Some(Region::Piaui), 22),
            Test::Case(Some(Region::RioGrandeDoNorte), 24),
            Test::Case(Some(Region::RioGrandeDoSul), 43),
            Test::Case(Some(Region::RioDeJaneiro), 33),
            Test::Case(Some(Region::Rondonia), 11),
            Test::Case(Some(Region::Roraima), 14),
            Test::Case(Some(Region::SantaCatarina), 42),
            Test::Case(Some(Region::SaoPaulo), 35),
            Test::Case(Some(Region::Sergipe), 28),
            Test::Case(Some(Region::Tocantins), 17),
        ]
        .iter()
        .for_each(move |it| match it {
            Test::Case(region, code) => assert_eq!(region, &Region::parse(code)),
        });

        assert_eq!(None, Region::parse(&33333));
    }

    enum Test {
        Case(Option<Region>, u16),
    }
}
