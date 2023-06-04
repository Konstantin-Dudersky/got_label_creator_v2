use std::error::Error;

use crate::domain;

const FILE: &str = "export.csv";

pub fn create_csv(data: domain::Data) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(FILE)?;

    let mut record_creator =
        RecordCreator::new(&data.table_number, &data.table_name, data.offset);

    wtr.write_record(record_creator.create_record_1())?;
    wtr.write_record(record_creator.create_record_2())?;
    wtr.write_record(record_creator.create_record_3())?;
    wtr.write_record(record_creator.create_record_4())?;

    for order in 0..data.amount {
        for item in &data.items {
            let record = record_creator.create_record_for_item(&item, order);
            wtr.write_record(record)?;
        }
    }

    wtr.flush()?;

    Ok(())
}

struct RecordCreator<'a> {
    table_number: &'a str,
    table_name: &'a str,
    word_offset: u32,
    bit_offset: u32,
    prev_order: u32,
}

impl<'a> RecordCreator<'a> {
    fn new(table_number: &'a str, table_name: &'a str, offset: u32) -> Self {
        Self {
            table_number,
            table_name,
            word_offset: offset,
            bit_offset: 0,
            prev_order: 0,
        }
    }

    fn create_empty_record() -> Vec<String> {
        vec![String::from(""); 20]
    }

    fn create_record_1(&self) -> Vec<String> {
        let mut record = Self::create_empty_record();
        record[0] = clean_string(self.table_number).to_string();
        record[1] = clean_string(self.table_name).to_string();
        record
    }

    fn create_record_2(&self) -> Vec<String> {
        Self::create_empty_record()
    }

    fn create_record_3(&self) -> Vec<String> {
        Self::create_empty_record()
    }

    fn create_record_4(&self) -> Vec<String> {
        let mut record = Self::create_empty_record();
        record[1] = String::from("Label Name");
        record[2] = String::from("Data Type");
        record[3] = String::from("Assign (Device)");
        record[4] = String::from("Comment");
        record[5] = String::from("Comment2");
        record[6] = String::from("Comment3");
        record[7] = String::from("Comment4");
        record[8] = String::from("Comment5");
        record[9] = String::from("Comment6");
        record[10] = String::from("Comment7");
        record[11] = String::from("Comment8");
        record[12] = String::from("Comment9");
        record[13] = String::from("Comment10");
        record[14] = String::from("Comment11");
        record[15] = String::from("Comment12");
        record[16] = String::from("Comment13");
        record[17] = String::from("Comment14");
        record[18] = String::from("Comment15");
        record[19] = String::from("Comment16");
        record
    }

    fn create_record_for_item(
        &mut self,
        item: &'a domain::ItemInStruct,
        order: u32,
    ) -> Vec<String> {
        // выравниваем, если прошлая итерация закончилась битами
        if order != self.prev_order {
            if self.bit_offset > 0 {
                self.bit_offset = 0;
                self.word_offset += 1;
            }
            self.prev_order = order;
        }

        let name = clean_string(item.name);
        let name = format!("{}_{}", name, order);
        let data_type = clean_string(item.data_type).to_string();

        let mut record = Self::create_empty_record();
        record[1] = name;
        record[2] = data_type;

        if item.data_type.contains("Bit") {
            record[3] = format!(
                "D{}.b{}",
                self.word_offset.to_string(),
                self.bit_offset
            );
            self.bit_offset += item.size as u32;
        } else {
            if self.bit_offset > 0 {
                self.word_offset += 1;
                self.bit_offset = 0;
            }
            record[3] = format!("D{}", self.word_offset);
            self.word_offset += item.size as u32;
        }

        record
    }
}

fn clean_string<'a>(input_str: &'a str) -> &'a str {
    let data_type = input_str.trim();
    let data_type = data_type.strip_suffix("\n").unwrap_or(data_type);
    let data_type = data_type.strip_suffix("\r\n").unwrap_or(data_type);
    data_type
}

#[cfg(test)]
mod test {
    // тесты запускать по одному, и визуально проверять с файлами
    // в папке ./tests

    use super::*;
    use crate::builder;

    #[test]
    fn test1() {
        let mut builder = builder::Builder::new();
        builder.set_table_number("1");
        builder.set_table_name("test1");
        builder.set_offset(1000);
        builder.set_amount(2);
        builder.add_item("bool1", "Bit", 1);
        builder.add_item("bool2", "Bit", 1);
        builder.add_item("int3", "Signed BIN16", 1);
        builder.add_item("float4", "Real(32bit)", 2);
        builder.add_item("bool_arr5", "Bit[0..7]", 8);

        let data = builder.build();

        create_csv(data).unwrap();
    }
}
