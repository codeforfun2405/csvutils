pub mod dedup;
pub mod filter;
pub mod merge;

pub trait Processor {
    fn process(&self) -> anyhow::Result<()>;
}

pub struct CSVData {
    pub headers: Vec<String>,   // the header fields
    pub data: Vec<Vec<String>>, // csv record data
}

impl CSVData {
    pub fn new(headers: Vec<String>, data: Vec<Vec<String>>) -> Self {
        Self { headers, data }
    }
}

pub fn read_csv(path: String) -> anyhow::Result<CSVData> {
    let mut csv_data = csv::Reader::from_path(path)?;
    let headers = csv_data.headers().cloned()?;

    let mut data_records: Vec<Vec<String>> = vec![];
    for record in csv_data.records() {
        let r = record?;
        let data = r.iter().map(|v| v.to_string()).collect::<Vec<String>>();

        data_records.push(data);
    }

    let hdrs = headers
        .iter()
        .map(|hdr| hdr.to_string())
        .collect::<Vec<String>>();

    anyhow::Ok(CSVData::new(hdrs, data_records))
}

pub fn writer_csv(path: String, csv_data: CSVData) -> anyhow::Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(csv_data.headers)?;
    for r in csv_data.data {
        writer.write_record(r)?;
    }
    writer.flush()?;

    anyhow::Ok(())
}
