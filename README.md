# got_label_creator

Генерирование таблицы тегов для панелей Mitsubishi для массивов структур.

![](https://github.com/Konstantin-Dudersky/got_label_creator_v2/blob/master/docs/Screenshot_20230604_120116.png?raw=true)

Программа запускается без установки. В поле "Размер" заносится кол-во бит (для типов данных, которые состоят из Bit) или кол-во слов. По кнопке "Создать файл" рядом с исполняемым файлом создается файл export.csv.

## Сборка

Linux:

```
cargo build --release
```

Windows:

```
cargo build --release
```
