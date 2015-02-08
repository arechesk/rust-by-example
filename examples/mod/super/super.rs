fn function() {
    println!("вызывает `function()`");
}

mod my {
    pub fn indirect_call() {
        // Давайте вызовем все функции с именем `function` в этой области видимости
        print!("вызвана `my::indirect_call()`, которая\n> ");

        // `my::function` можно вызвать напрямую
        function();

        {
            // Это свяжется с `cool::function` из области видимости *крэйта*
            // В данном случае, область видимости крэйта — самая широкая
            use cool::function as root_cool_function;

            print!("> ");
            root_cool_function();
        }

        {
            // `self` ссылается на область видимости модуля, в данном случае — `my`
            use self::cool::function as my_cool_function;

            print!("> ");
            my_cool_function();
        }

        {
            // `super` ссылается на родительскую область видимости, т.е снаружи
            // модуля `my`
            use super::function as root_function;

            print!("> ");
            root_function();
        }
    }

    fn function() {
        println!("вызывает `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("вызывает `my::cool::function()`");
        }
    }
}

mod cool {
    pub fn function() {
        println!("вызывает `cool::function()`");
    }
}

fn main() {
    my::indirect_call();
}
