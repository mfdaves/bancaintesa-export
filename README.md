Rust library for parsing the useless export xlsx that banca intesa provides to its customers.

## Features 
It just returns a Vec<BancaIntesaRecord> which provides for each row: date, operation, details, category, currency and amount. 
I use hardcoded values like sheet name "Lista Operazione" and skip the first 18 human excel rows, because they added a very useful header..
I could add a config.toml maybe
I don't know if in the future they'll change column names or header height.
They also have a not trimmed column: "Categoria ".
I saw they've an endpoint called: "/getOperazione", maybe it can be use (which is the raw export data) in the browser to have a cleaner output
using SheetJS or something similar.  

## Why?
Maybe it could be useful to insert it or convert it in your budget tracker app.

## Usage

``` rust
fn main(){
  let path = "/path/to/your/export.xlsx";

  let records = parse_bank_export(path).unwrap();

  let incomes: f64 = records
      .iter()
      .filter(|x| x.amount > 0.0)
      .map(|x| x.amount)
      .sum();

}
```
