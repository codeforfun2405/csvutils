use anyhow::anyhow;

use super::{read_csv, writer_csv, CSVData, Processor};
use crate::opts::merge::MergeOptions;

impl Processor for MergeOptions {
    fn process(&self) -> anyhow::Result<()> {
        if self.inputs.len() < 1 {
            return Err(anyhow!("empty inputs"));
        }

        if self.output == "" {
            return Err(anyhow!("empty output"));
        }

        let mut data: Vec<Vec<String>> = Vec::new();
        let mut headers = Vec::new();
        for i in self.inputs.clone() {
            let csv_data = read_csv(i.to_string())?;
            if headers.len() == 0 {
                headers = csv_data.headers;
            }

            data.extend_from_slice(csv_data.data.as_slice());
        }

        writer_csv(self.output.to_string(), CSVData::new(headers, data))?;

        anyhow::Ok(())
    }
}
