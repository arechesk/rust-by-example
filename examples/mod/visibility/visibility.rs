fn function() {
    println!("вызвана `function()`");
}

mod my {
    // Публичная функция
    pub fn function() {
        println!("вызвана `my::function()`");
    }

    // Приватная функция
    fn private_function() {
        println!("вызывает `my::private_function()`");
    }

    // Элементам модуля доступны другие элементы того же модуля
    pub fn indirect_access() {
        print!("вызвана `my::indirect_access()`, которая\n> ");

        // вне зависимости от их видимости
        private_function();
    }

    // Публичный модуль
    pub mod nested {
        pub fn function() {
            println!("вызвана `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("вызвана `my::nested::private_function()`");
        }
    }

    // Приватный модуль
    mod inaccessible {
        #[allow(dead_code)]
        pub fn public_function() {
            println!("вызвана `my::inaccessible::public_function()`");
        }
    }
}

fn main() {
    // К публичным элементам есть доступ
    my::function();

    // модули решают проблему нескольких элементов с одинаковым именем
    function();

    // Приватные элементы модуля недоступны напрямую
    // Ошибка! `private_function` приватна
    //my::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать это строку

    my::indirect_access();

    // Публичные элементы вложенных публичных модулей доступны за пределами
    // родительского модуля
    my::nested::function();

    // но приватные элементы вложенных публичных модулей недоступны
    // Ошибка! `private_function` приватна
    //my::nested::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать это строку

    // Элементы внутри приватных вложенных модулей недоступны вне зависимости
    // от их видиости
    // Ошибка! `inaccessible` — приватный модуль
    //my::inaccessible::public_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать это строку
}
