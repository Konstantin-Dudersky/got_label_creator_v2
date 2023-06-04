#![windows_subsystem = "windows"]

use got_label_creator_v2::builder::Builder;
use got_label_creator_v2::create_csv;

slint::include_modules!();

fn main() {
    let app = Main::new().unwrap();
    app.global::<Logic>().on_create_file_clicked(|settings| {
        let mut data_builder = Builder::new();
        data_builder.set_table_number(settings.table_number.as_str());
        data_builder.set_table_name(settings.table_name.as_str());
        data_builder.set_offset(settings.offset as u32);
        data_builder.set_amount(settings.amount as u32);
        data_builder.add_item(
            &settings.item_0.name,
            &settings.item_0.data_type,
            settings
                .item_0
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_1.name,
            &settings.item_1.data_type,
            settings
                .item_1
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_2.name,
            &settings.item_2.data_type,
            settings
                .item_2
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_3.name,
            &settings.item_3.data_type,
            settings
                .item_3
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_4.name,
            &settings.item_4.data_type,
            settings
                .item_4
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_5.name,
            &settings.item_5.data_type,
            settings
                .item_5
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_6.name,
            &settings.item_6.data_type,
            settings
                .item_6
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_7.name,
            &settings.item_7.data_type,
            settings
                .item_7
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_8.name,
            &settings.item_8.data_type,
            settings
                .item_8
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        data_builder.add_item(
            &settings.item_9.name,
            &settings.item_9.data_type,
            settings
                .item_9
                .size
                .try_into()
                .expect("Неправильное значение размера"),
        );
        let data = data_builder.build();

        create_csv::create_csv(data).unwrap();
    });
    app.run().unwrap()
}
