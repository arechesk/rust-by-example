// Привязать путь `deeply::nested::function` к `other_function`
use deeply::nested::function as other_function;

fn function() {
    println!("вызвана `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("вызвана `deeply::nested::function()`")
        }
    }
}

fn main() {
    // Упрощённый доступ к `deeply::nested::function`
    other_function();

    println!("Входим в блок");
    {
        // Эквивалентно `use deeply::nested::function as function`
        // Эта `function` скроет собой внешнюю
        use deeply::nested::function;

        function();

        println!("Выходим из блока");

        // у привязок `use` локальная область видимости, в данном случае
        // внешная `function` скрыта лишь в этой области видимости
    }

    function();
}
