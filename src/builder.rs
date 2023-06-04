use crate::domain;

#[derive(Default)]
pub struct Builder<'a> {
    table_number: Option<&'a str>,
    table_name: Option<&'a str>,
    offset: Option<u32>,
    amount: Option<u32>,
    items: Vec<domain::ItemInStruct<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        let mut data: Self = Default::default();
        data.items = vec![];
        data
    }

    pub fn set_table_number(&mut self, table_number: &'a str) {
        self.table_number = Some(table_number);
    }

    pub fn set_table_name(&mut self, table_name: &'a str) {
        self.table_name = Some(table_name);
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = Some(amount);
    }

    pub fn add_item<'add_item>(
        &mut self,
        name: &'a str,
        data_type: &'a str,
        size: i8,
    ) {
        if name.len() == 0 {
            return;
        }
        let new_item = domain::ItemInStruct {
            name,
            data_type,
            size,
        };
        self.items.push(new_item);
    }

    pub fn build(self) -> domain::Data<'a> {
        domain::Data::new(
            String::from(self.table_number.expect("Не задан номер таблицы")),
            String::from(self.table_name.expect("Не задано название таблицы")),
            self.offset.expect("Не задано начальное смещение"),
            self.amount.expect("Не задано кол-во структур"),
            self.items,
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn build() {
        let mut builder = Builder::new();
        builder.set_table_number("1");
        builder.set_table_name("table_name");
        builder.add_item("item_1", "int", 1);

        let data = builder.build();

        assert_eq!(data.table_name, "table_name");
        assert_eq!(data.table_number, "1");
        assert_eq!(data.items[0].name, "item_1");
        assert_eq!(data.items[0].data_type, "int");
        assert_eq!(data.items[0].size, 1);
    }
}
