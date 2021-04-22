use std::fmt::Debug;

// For more details see [here](https://ibge.gov.br/explica/codigos-dos-municipios.php)
// and [here](https://gitlab.procc.fiocruz.br/mave/repo/-/tree/master/Dados/InfoGripe#alerta-de-situa%C3%A7%C3%A3o-com-base-no-n%C3%BAmero-de-novos-casos-semanais)

#[derive(Debug, PartialEq)]
enum Region {
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
    pub(crate) fn parse(code: &u16) -> Result<Region, ()> {
        match code {
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
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::Region;

    #[test]
    fn it_parse_regions() -> Result<(), Box<dyn Error>> {
        vec![
            Test::Case(Ok(Region::Acre), 12),
            Test::Case(Ok(Region::Alagoas), 27),
            Test::Case(Ok(Region::Amapa), 16),
            Test::Case(Ok(Region::Amazonas), 13),
            Test::Case(Ok(Region::Bahia), 29),
            Test::Case(Ok(Region::Ceara), 23),
            Test::Case(Ok(Region::DistritoFederal), 53),
            Test::Case(Ok(Region::EspiritoSanto), 32),
            Test::Case(Ok(Region::Goias), 52),
            Test::Case(Ok(Region::Maranhao), 21),
            Test::Case(Ok(Region::MatoGrosso), 51),
            Test::Case(Ok(Region::MatoGrossoDoSul), 50),
            Test::Case(Ok(Region::MinasGerais), 31),
            Test::Case(Ok(Region::Para), 15),
            Test::Case(Ok(Region::Paraiba), 25),
            Test::Case(Ok(Region::Parana), 41),
            Test::Case(Ok(Region::Pernambuco), 26),
            Test::Case(Ok(Region::Piaui), 22),
            Test::Case(Ok(Region::RioGrandeDoNorte), 24),
            Test::Case(Ok(Region::RioGrandeDoSul), 43),
            Test::Case(Ok(Region::RioDeJaneiro), 33),
            Test::Case(Ok(Region::Rondonia), 11),
            Test::Case(Ok(Region::Roraima), 14),
            Test::Case(Ok(Region::SantaCatarina), 42),
            Test::Case(Ok(Region::SaoPaulo), 35),
            Test::Case(Ok(Region::Sergipe), 28),
            Test::Case(Ok(Region::Tocantins), 17),
        ]
        .iter()
        .for_each(move |it| match it {
            Test::Case(region, code) => assert_eq!(region, &Region::parse(code)),
        });

        Ok(())
    }

    enum Test {
        Case(Result<Region, ()>, u16),
    }
}
