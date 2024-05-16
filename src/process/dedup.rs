use super::{read_csv, writer_csv, CSVData, Processor};
use crate::opts::dedup::DedupOptions;

use anyhow::anyhow;
use std::collections::HashSet;

impl Processor for DedupOptions {
    fn process(&self) -> anyhow::Result<()> {
        let mut origin_data = read_csv(self.input.to_string())?;

        let hdrs = origin_data.headers.clone();
        let field_idx = origin_data.headers.iter().position(|x| *x == self.field);

        match field_idx {
            Some(idx) => {
                let mut set = HashSet::new();
                origin_data.data.retain(|r| set.insert(r[idx].to_string()));
            }
            None => {
                return Err(anyhow!("field {} is not exist", self.field));
            }
        }

        writer_csv(
            self.output.to_string(),
            CSVData::new(hdrs, origin_data.data),
        )?;

        anyhow::Ok(())
    }
}
