use got_label_creator_v2::{test_fn1, Settings};

slint::include_modules!();

fn main() {
    let app = Main::new().unwrap();
    app.global::<Logic>().on_create_file_clicked(|settings| {
        let received_settings = Settings {
            table_number: settings.table_number.as_str(),
            table_name: settings.table_name.as_str(),
        };
        test_fn1(received_settings);
    });
    app.run().unwrap()
}
