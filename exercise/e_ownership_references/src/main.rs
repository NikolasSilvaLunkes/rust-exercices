// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables, dead_code)]

const CONSTANTE:i32 = 2;

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    inspect(&arg);
    let mut s:String = String::from("batatinha");
    no_borrow_test(s.clone());
    println!("{}",s);
    borrow_test(&mut s);
    println!("{}",s);
    

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(arg.clone()) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }


    let mut structest:RedFox = RedFox::new();
    println!("The fox life is: {}",structest.life);

    structest.eat();

    println!("The fox life has changed to: {}",structest.life);
    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);

    print_noise(structest);         


}

pub fn inspect(s:&str) {
    if s.ends_with("s"){
        println!("This string ends with s");
    }else{
        println!("This string does not end with s");
    }
}

pub fn change(s:&mut String) {
    if !s.ends_with("s"){
        s.push_str("s")
    }
}

pub fn eat(s:String) -> bool {
    return s.starts_with("b") && s.contains("a") 
        
}

pub fn bedazzle(s:&mut String){
    *s = String::from("sparkly");
}

pub fn borrow_test(s: &mut String){
    *s= s.to_uppercase()+"s";
    println!("{}",s);
}

pub fn no_borrow_test(s:String){
    let mut s:String = s;
    s = s.to_uppercase();
    println!("{}",s);
}



//Declaring struct (similar to class) called RedFox
struct RedFox {
    enemy: bool,
    life: i32,
}

//Implementations (Methods) for the struct redfox
impl RedFox {
    fn new() -> Self {
        Self { enemy: true, life: 70 }
    }
    fn eat(&mut self) {
        self.life += 10;
    }
}

//Declarating the trait noisy
trait Noisy {
    fn get_noise(&self) ->&str;
}

//Implementing noisy in redfox
impl Noisy for RedFox {
    fn get_noise(&self) -> &str { "Meow?" }
}


//A generic function for any struct that implements noisy
fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}


 