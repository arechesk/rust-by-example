fn main() {
    // Целочисленное сложение
    println!("1 + 2 = {}", 1u32 + 2);

    // Целочисленное вычитание
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Попробуйте изменить `1i32` на `1u32` чтобы понять, почему тип имеет значение

    // Булева логика
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Битовые операции
    println!("0011 AND 0101 is {:04t}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04t}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04t}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Используйте подчеркивания, чтобы улучшить читаемость
    println!("One million is written as {}", 1_000_000u32);
}
