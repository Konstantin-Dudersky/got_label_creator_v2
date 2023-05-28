use got_label_creator_v2::builder::Builder;

slint::include_modules!();

fn main() {
    let app = Main::new().unwrap();
    app.global::<Logic>().on_create_file_clicked(|settings| {
        // let received_settings = Data {
        //     table_number: settings.table_number.to_string(),
        //     table_name: settings.table_name.to_string(),
        // };
        // test_fn1(received_settings);
        let data = Builder::new()
            .set_table_number(String::from("1"))
            .set_table_name(String::from("table_name"))
            .build();
        println!("data: {:?}", data);
    });
    app.run().unwrap()
}
