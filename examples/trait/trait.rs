trait Animal {
    // Cигнатура статического метода, `Self` ссылается на реализующий тип
    fn new(name: &'static str) -> Self;

    // Методы экземляра, только сигнатуры
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Типаж может содержать определение метода по-умолчанию
    fn talk(&self) {
        // Эти определения могут использовать другие методы, объявленные
        // в том же самом типаже
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog { name: &'static str }

impl Dog {
    fn wag_tail(&self) {
        println!("{} wags tail", self.name);
    }
}

// Реализация типажа `Animal` для типа `Dog`
impl Animal for Dog {
    // `Self` заменён на реализующий тип: `Dog`
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof!"
    }

    // Определённые по умолчанию методы могут быть переопределены
    fn talk(&self) {
        // Методы трейта могут использовать методы реализующего типа
        self.wag_tail();

        println!("{} says {}", self.name, self.noise());
    }
}

struct Sheep { naked: bool, name: &'static str }

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Методы типа могут использовать методы типажа, реализованного для этого типа
            println!("{} is already naked!", self.name());
        } else {
            println!("{} gets a haircut", self.name);

            self.talk();
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaah"
        } else {
            "baaaaaaaaaaaah"
        }
    }
}

fn main() {
    // Анотация типа в данном случае необходима
    let mut dolly: Sheep = Animal::new("Dolly");
    let spike: Dog = Animal::new("Spike");
    // ЗАДАНИЕ ^ Попробуйте убрать аннотации типа

    dolly.shear();

    spike.talk();
    dolly.talk();
}
