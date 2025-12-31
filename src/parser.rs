use calamine::{
    Error, RangeDeserializerBuilder, Reader, Xlsx, deserialize_as_date_or_none, open_workbook,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const WORKSHEET_NAME: &str = "Lista Operazione";

#[derive(Debug, Deserialize, Serialize)]
pub struct BancaIntesaRecord {
    #[serde(rename = "Data", deserialize_with = "exceldt_to_date")]
    date: NaiveDate,

    #[serde(rename = "Operazione")]
    operation: String,

    #[serde(rename = "Dettagli")]
    details: String,

    #[serde(rename = "Categoria")]
    category: String,

    #[serde(rename = "Valuta")]
    currency: String,

    #[serde(rename = "Importo")]
    amount: f64,
}

pub fn parse_bank_export<S: AsRef<Path>>(path: S) -> Result<Vec<BancaIntesaRecord>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path.as_ref())?;
    let range = workbook.worksheet_range(WORKSHEET_NAME)?;
    let clean_range = range.range((18, 0), range.end().unwrap());

    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range(&clean_range)?;

    Ok(iter.filter_map(|record| record.ok()).collect())
}

pub fn exceldt_to_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_as_date_or_none(deserializer)?
        .ok_or_else(|| serde::de::Error::custom("Invalide Date"))
}
