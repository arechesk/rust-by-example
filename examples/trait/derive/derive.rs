// Кортежеподобная структура, которую можно сравнить
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// Кортежеподобная структура, которую можно напечатать
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// Просто кортежеподобная структура
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Ошибка! `Seconds` нельзя напечатать, так как он не реализует типаж `Debug`
    //println!("One second looks like: {:?}", _one_second);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка: `Seconds` нельзя сравнивать, так как он не реализует типаж `PartialEq`
    //let _this_is_true = _one_second == _one_second;
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    let foot = Inches(12);

    println!("One foot === {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("one foot is {} than one meter", cmp);
}
