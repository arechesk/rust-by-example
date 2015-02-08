fn function() {
    println!("called `function()`");
}

// Модуль с именем `my`
mod my {
    // Модуль может содержать элементы вроде функций
    #[allow(dead_code)]
    fn function() {
        println!("called `my::function()`");
    }

    // Модули могут быть вложенными
    mod nested {
        #[allow(dead_code)]
        fn function() {
            println!("called `my::nested::function()`");
        }
    }
}

fn main() {
    function();

    // Элементы внутри модуля могут быть вызваны с использованием полного пути
    // Функция `println` живёт в модуле `stdio`
    // Модуль `stdio` живёт в модуле `io`
    // А модуль `io` живёт в крэйте `std`
    std::old_io::stdio::println("Hello World!");

    // Ошибка! Функция my::function — приватная
    my::function();
    // ЗАДАНИЕ ^ Закомментируйте эту строку
}
