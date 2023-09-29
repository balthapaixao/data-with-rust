fn integers() {
    let signed: i16 = -57;
    let unsigned: u16 = 57;

    println!("\nINTEGER TYPES");
    println!("Signed: {}", signed);
    println!("Unsigned: {}", unsigned);
}

fn floats() {
    let float: f32 = 6.4;
    let double: f64 = 6.4;

    println!("\nFLOAT TYPES");
    println!("Float: {}", float);
    println!("Double: {}", double);
}

fn booleans() {
    let t = true;
    let f: bool = false;

    println!("\nBOOLEAN TYPES");
    println!("True: {}", t);
    println!("False: {}", f);
}

fn characters() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("\nCHARACTER TYPES");
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
}

fn strings() {
    let s1 = "Hello, world!";
    let s2 = String::from("Hello, world!");

    println!("\nSTRING TYPES");
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn tuples() {
    let tuple: (i32, f64, bool) = (500, 6.4, true);

    println!("\nTUPLE TYPES");
    println!("tuple: {:?}", tuple);
}

fn arrays() {
    let array: [i8; 4] = [1, 8, 9, 8];

    println!("\nARRAY TYPES");
    println!("array: {:?}", array);
}
fn structs() {
    // named fields
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        likes_football: bool,
    }
    #[derive(Debug)]
    struct ShirtColor(u8, u8, u8);

    let baltha = Person {
        name: String::from("Balthazar"),
        age: 23,
        likes_football: true,
    };
    let baltha_shirt_color = ShirtColor(123, 213, 132);

    println!("\nSTRUCT TYPES");
    println!("baltha: {:?}", baltha);
    println!("baltha_shirt_color: {:?}", baltha_shirt_color);
}

fn enums() {
    #[derive(Debug)]
    enum PossibleTeams {
        Vasco,
        CRVG,
        GiganteDaColina,
        VascoDaGama,
    }

    #[derive(Debug)]
    enum Options<T> {
        Some(T),
        None,
    }

    let team = PossibleTeams::VascoDaGama;
    let option: Options<i32> = Options::Some(5);
    println!("\nENUM TYPES");
    println!("team: {:?}", team);
    println!("option: {:?}", option);
}

fn impls() {
    #[derive(Debug)]
    struct Car {
        model: String,
    }

    impl Car {
        fn new(model: &str) -> Car {
            return Car {
                model: model.to_string(),
            };
        }
        fn accelerate(&self) {
            println!("{} is accelerating!", self.model);
        }
    }

    let car = Car::new("Fusca");

    println!("\nIMPL TYPES");
    println!("car: {:?}", car);
    car.accelerate();
}

fn traits() {
    trait CanRevFast {
        fn revengine(&self);
        fn is_fast(&self);
    }

    #[derive(Debug)]
    struct Car {
        model: String,
    }
    impl Car {
        fn new(name: &str) -> Car {
            return Car {
                model: name.to_string(),
            };
        }
        fn accelerate(&self) {
            println!("{} is accelerating!", self.model);
        }
    }

    //guarantee that Car implements Can Reverse Fast
    impl CanRevFast for Car {
        fn revengine(&self) {
            println!("{} is reverse!", self.model);
        }
        fn is_fast(&self) {
            println!("{} is fast!", self.model);
        }
    }

    let car = Car::new("Fusca");

    println!("\nTRAITS TYPES");
    println!("car: {:?}", car);
    car.accelerate();
    car.revengine();
    car.is_fast();
}

fn main() {
    integers();
    floats();
    booleans();
    characters();
    strings();
    tuples();
    arrays();
    structs();
    enums();
    impls();
    traits();
}
