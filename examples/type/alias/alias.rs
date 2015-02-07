// `NanoSecond` это новое имя для `u64`
type NanoSecond = u64;
type Metre = u64;

// Используйте этот атрибут, чтобы не выводить предупреждение
#[allow(non_camel_case_types)]
type u64_t = u64;
// ^ Попробуйте удалить атрибут

fn main() {
    // `NanoSecond` = `Metre` = `u64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as u64_t;
    let metres: Metre = 2 as u64_t;

    // Обратите внимание, что псевдонимы *не предоставляют* никакой
    // дополнительной типобезопасности, так как *не являются* новыми типами
    println!("{} наносекунд + {} метра = {} каких единиц?",
             nanoseconds,
             metres,
             nanoseconds + metres);
}
