// Связать `liberty`, импортировать элементы как модуль `erty`
extern crate erty;

fn main() {
    erty::public_function();

    // Ошибка! `private_function` приватна
    //erty::private_function();

    erty::indirect_access();
}
