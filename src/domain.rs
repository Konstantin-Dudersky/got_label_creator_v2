#[derive(Debug)]
struct _ItemInStruct {
    _name: String,
    _data_type: String,
    _size: i8,
}

#[derive(Debug)]
pub struct Data {
    pub table_number: String,
    pub table_name: String,
}

impl Data {
    pub fn new(table_number: String, table_name: String) -> Self {
        Self {
            table_number,
            table_name,
        }
    }
}
