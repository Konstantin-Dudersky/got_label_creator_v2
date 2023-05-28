use crate::domain;

#[derive(Default)]
pub struct Builder {
    table_number: Option<String>,
    table_name: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_table_number(mut self, table_number: String) -> Self {
        self.table_number = Some(table_number);
        self
    }

    pub fn set_table_name(mut self, table_name: String) -> Self {
        self.table_name = Some(table_name);
        self
    }

    pub fn build(self) -> domain::Data {
        domain::Data::new(
            self.table_number.expect("Не задан номер таблицы"),
            self.table_name.expect("Не задано название таблицы"),
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn build() {
        let data = Builder::new()
            .set_table_number(String::from("1"))
            .set_table_name(String::from("table_name"))
            .build();
        assert_eq!(data.get_table_name(), "table_name");
        assert_eq!(data.get_table_number(), "1");
    }
}
