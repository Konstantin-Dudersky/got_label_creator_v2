#![windows_subsystem = "windows"]

use got_label_creator_v2::builder::Builder;
use got_label_creator_v2::create_csv;
use got_label_creator_v2::log;

slint::include_modules!();

fn main() {
    log::init();

    let app = Main::new().unwrap();
    app.global::<Logic>().on_create_file_clicked(|settings| {
        let data = Builder::new()
            .set_table_number(settings.table_number.to_string())
            .set_table_name(settings.table_name.to_string())
            .build();
        create_csv::create_csv(data).unwrap();
    });
    app.run().unwrap()
}
