#[derive(Debug)]
struct ItemInStruct {
    name: String,
    data_type: String,
    size: i8,
}

#[derive(Debug)]
pub struct Data {
    table_number: String,
    table_name: String,
}

impl Data {
    pub fn new(table_number: String, table_name: String) -> Self {
        Self {
            table_number,
            table_name,
        }
    }

    pub fn get_table_number(&self) -> &str {
        &self.table_number
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name
    }
}
