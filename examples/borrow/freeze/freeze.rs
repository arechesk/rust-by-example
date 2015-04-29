fn main() {
    let mut _integer = 5i32;

    {
        // Занять `integer`
        let _ref_to_integer = &_integer;

        // Ошибка `integer` заморожен в этой области видимости
        _integer = 4;
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        // `ref_to_integer` покидает область видимости
    }

    // Ок! `integer` не заморожен в этой области видимости
    _integer = 4;
}
