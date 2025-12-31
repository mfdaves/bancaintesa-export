use calamine::{
    Error, RangeDeserializerBuilder, Reader, Xlsx, deserialize_as_date_or_none, open_workbook,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const WORKSHEET_NAME: &str = "Lista Operazione";
pub const USELESS_ROWS: u32 = 18;

#[derive(Debug, Deserialize, Serialize)]
pub struct BancaIntesaRecord {
    #[serde(rename = "Data", deserialize_with = "exceldt_to_date")]
    pub date: NaiveDate,

    #[serde(rename = "Operazione")]
    pub operation: String,

    #[serde(rename = "Dettagli")]
    pub details: String,

    // see it's right to keep it as "Categoria "
    #[serde(rename = "Categoria ")]
    pub category: String,

    #[serde(rename = "Valuta")]
    pub currency: String,

    #[serde(rename = "Importo")]
    pub amount: f64,
}

pub fn parse_bank_export<S: AsRef<Path>>(path: S) -> Result<Vec<BancaIntesaRecord>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path.as_ref())?;
    let range = workbook.worksheet_range(WORKSHEET_NAME)?;
    let clean_range = range.range((USELESS_ROWS, 0), range.end().unwrap());

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
