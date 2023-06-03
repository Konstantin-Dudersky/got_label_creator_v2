use std::error::Error;

use tracing;

use crate::domain;

const FILE: &str = "export.csv";

pub fn create_csv(data: domain::Data) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(FILE)?;

    let rec_gen = RecordCreator::new(&data);

    wtr.write_record(rec_gen.create_record_1())?;
    wtr.write_record(rec_gen.create_record_2())?;
    wtr.write_record(rec_gen.create_record_3())?;
    wtr.write_record(rec_gen.create_record_4())?;

    wtr.flush()?;

    tracing::info!("file creating tracing");

    Ok(())
}

struct RecordCreator<'a> {
    data: &'a domain::Data,
}

impl<'a> RecordCreator<'a> {
    fn new(data: &'a domain::Data) -> Self {
        Self { data }
    }

    fn create_empty_record() -> Vec<&'a str> {
        vec![""; 20]
    }

    fn create_record_1(&self) -> Vec<&'a str> {
        let mut record = Self::create_empty_record();
        record[0] = self.data.table_number.as_str();
        record[2] = self.data.table_name.as_str();
        record
    }

    fn create_record_2(&self) -> Vec<&'a str> {
        Self::create_empty_record()
    }

    fn create_record_3(&self) -> Vec<&'a str> {
        Self::create_empty_record()
    }

    fn create_record_4(&self) -> Vec<&'a str> {
        let mut record = Self::create_empty_record();
        record[1] = "Label Name";
        record[2] = "Data Type";
        record[3] = "Assign (Device)";
        record[4] = "Comment";
        record[5] = "Comment2";
        record[6] = "Comment3";
        record[7] = "Comment4";
        record[8] = "Comment5";
        record[9] = "Comment6";
        record[10] = "Comment7";
        record[11] = "Comment8";
        record[12] = "Comment9";
        record[13] = "Comment10";
        record[14] = "Comment11";
        record[15] = "Comment12";
        record[16] = "Comment13";
        record[17] = "Comment14";
        record[18] = "Comment15";
        record[19] = "Comment16";
        record
    }
}
