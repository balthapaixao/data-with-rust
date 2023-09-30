mod modules;
mod some_module {
    //public function
    pub fn some_public_function() {
        println!("Hey everyone! The answer is {}", CONSTANT);
    }
    //private function
    fn some_private_function() {
        println!("The password is: 🥔");
    }
    struct SomeStruct {
        potato: String,
    }

    const CONSTANT: u32 = 42;
}

fn main() {
    some_module::some_public_function();
    modules::utils::function();
}
