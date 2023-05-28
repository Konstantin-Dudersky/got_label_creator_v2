pub fn test_fn1(table_number: Settings) {
    println!("Button clicked, {:?}", table_number);
}

#[derive(Debug)]
pub struct Settings<'a> {
    pub table_number: &'a str,
    pub table_name: &'a str,
}
