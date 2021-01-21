extern crate csv;
extern crate serde;

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
pub struct SerieTemporalComEstivativasRecentesEntry {
    #[serde(rename = "data de publicação")]
    data_de_publicacao: String,
    #[serde(rename = "UF")]
    uf: String,
    #[serde(rename = "Unidade da Federação")]
    unidade_da_federacao: String,
    #[serde(rename = "Tipo")]
    tipo: String,
    #[serde(rename = "dado")]
    dado: String,
    #[serde(rename = "escala")]
    escala: String,
    #[serde(rename = "Ano epidemiológico")]
    ano_epidemiologico: String,
    #[serde(rename = "Semana epidemiológica")]
    semana_epidemiologica: String,
    #[serde(rename = "Situação do dado")]
    situacao_do_dado: String,
    #[serde(rename = "Casos semanais reportados até a última atualização")]
    casos_semanais_reportados_ate_a_ultima_atualizacao: String,
    #[serde(rename = "limite inferior da estimativa")]
    limite_inferior_da_estimativa: String,
    #[serde(rename = "casos estimados")]
    casos_estimados: String,
    #[serde(rename = "média móvel")]
    media_movel: String,
    #[serde(rename = "limite superior da estimativa")]
    limite_superior_da_estimativa: String,
    #[serde(rename = "Percentual em relação ao país")]
    percentual_em_relacao_ao_pais: String,
    #[serde(rename = "População")]
    populacao: String,
    #[serde(rename = "limiar pré-epidêmico")]
    limiar_pre_epidemico: String,
    #[serde(rename = "intensidade alta")]
    intensidade_alta: String,
    #[serde(rename = "intensidade muito alta")]
    intensidade_muito_alta: String,
    #[serde(rename = "nível semanal")]
    nivel_semanal: String,
    #[serde(rename = "nível por média móvel")]
    nivel_por_media_movel: String,
}

#[cfg(test)]
mod tests {
    use crate::ports;
    use csv;
    use std::error::Error;

    #[test]
    fn it_loads_temporal_series_file() -> Result<(), Box<Error>> {
        let data = r#"
data de publicação;UF;Unidade da Federação;Tipo;dado;escala;Ano epidemiológico;Semana epidemiológica;Situação do dado;Casos semanais reportados até a última atualização;limite inferior da estimativa;casos estimados;média móvel;limite superior da estimativa;Percentual em relação ao país;População;limiar pré-epidêmico;intensidade alta;intensidade muito alta;nível semanal;nível por média móvel
2021-01-20;0;Brasil;País;srag;casos;2009;1;Dado estável. Sujeito a pequenas alterações.;0,0;;;;;;193543969;614,0;1588,0000000000002;2056,0;valor baixo;
2021-01-20;0;Brasil;País;srag;casos;2009;2;Dado estável. Sujeito a pequenas alterações.;0,0;;;0,0;;;193543969;614,0;1588,0000000000002;2056,0;valor baixo;Verde
"#;

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(data.as_bytes());

        for result in reader.deserialize() {
            let record: ports::srag::SerieTemporalComEstivativasRecentesEntry = result?;

            println!("{:?}", record);
        }

        Ok(())
    }
}
