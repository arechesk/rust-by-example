pub fn public_function() {
    println!("вызвана `public_function()` крэйта erty");
}

fn private_function() {
    println!("вызывает `private_function()` крэйта erty");
}

pub fn indirect_access() {
    print!("вызвана `indirect_access()` крэйта erty, которая\n> ");

    private_function();
}
