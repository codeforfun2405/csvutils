use anyhow::anyhow;

use super::{read_csv, writer_csv, CSVData, Processor};
use crate::opts::filter::FilterOptions;

impl Processor for FilterOptions {
    fn process(&self) -> anyhow::Result<()> {
        let (field, field_value, not_field, not_field_value) = process_fields(&self)?;

        // first get csv data by call read_csv
        let csv_data = read_csv(self.input.to_string())?;
        let headers = csv_data.headers.clone();

        // get the field index at the headers
        let field_idx = get_field_index(csv_data.headers, field)?;

        // get the not_field_index
        let not_field_idx = get_field_index(headers.clone(), not_field)?;

        let data: Vec<Vec<String>> = csv_data
            .data
            .iter()
            .filter(|record| {
                record[field_idx].contains(field_value.as_str())
                    && !record[not_field_idx].contains(not_field_value.as_str())
            })
            .map(|v| v.clone())
            .collect();

        writer_csv(self.output.to_string(), CSVData::new(headers, data))?;
        anyhow::Ok(())
    }
}

fn process_fields(opts: &FilterOptions) -> anyhow::Result<(String, String, String, String)> {
    let field_value: Vec<&str> = opts.field.split(':').collect();
    if field_value.len() < 2 {
        return Err(anyhow!("bad field: {}", opts.field));
    }

    let not_field_value: Vec<&str> = opts.not_field.split(':').collect();
    if not_field_value.len() < 2 {
        return Err(anyhow!("bad not_field: {}", opts.not_field));
    }

    let field = field_value[0].to_string();
    let field_value = field_value[1].to_string();

    let not_field = not_field_value[0].to_string();
    let not_field_value = not_field_value[1].to_string();

    if field == not_field {
        return Err(anyhow!(
            "field: {} and not_field: {} can not be same",
            field,
            not_field
        ));
    }

    anyhow::Ok((field, field_value, not_field, not_field_value))
}

fn get_field_index(headers: Vec<String>, f: String) -> anyhow::Result<usize> {
    match headers.iter().position(|h| *h == f) {
        Some(idx) => anyhow::Ok(idx),
        None => Err(anyhow!("field not found: {} in headers", f)),
    }
}
