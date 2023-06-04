#[derive(Debug)]
pub struct ItemInStruct<'a> {
    pub name: &'a str,
    pub data_type: &'a str,
    pub size: i8,
}

#[derive(Debug)]
pub struct Data<'a> {
    pub table_number: String,
    pub table_name: String,
    pub offset: u32,
    pub amount: u32,
    pub items: Vec<ItemInStruct<'a>>,
}

impl<'a> Data<'a> {
    /// Creates a new [`Data`].
    pub fn new(
        table_number: String,
        table_name: String,
        offset: u32,
        amount: u32,
        items: Vec<ItemInStruct<'a>>,
    ) -> Self {
        Self {
            table_number,
            table_name,
            offset,
            amount,
            items,
        }
    }
}
