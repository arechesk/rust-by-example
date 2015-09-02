// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]
fn main() {
    let decimal = 65.4321_f32;

    // Ошибка! Нет неявного преобразования
    let integer: u8 = decimal;
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    // Явное преобразование
    let integer = decimal as u8;
    let character = integer as char;

    println!("Приведение типов: {} -> {} -> {}", decimal, integer, character);
    // when casting any value to an unsigned type, T, 
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 вмещается в u16
    println!("1000 как u16 будет: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 как u8 будет : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 как u8 будет : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 будет : {}", 1000 % 256);

    // When casting to a signed type, the result is the same as 
    // first casting to the corresponding unsigned type then 
    // taking the two's complement.

    // Unless it already fits, of course.
    println!(" 128 как i16 будет: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 как i8 будет : {}", 128 as i8);

    // повторяем примеры
    // 1000 as u8 -> 232
    println!("1000 как i8 будет : {}", 1000 as i8);
    // and the two's complement of 232 is -24
    println!(" 232 как i8 будет : {}", 232 as i8);
}
