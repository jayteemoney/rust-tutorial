fn main() {
    println!("Hello, world!");
    // variables
    let name = "Alice"; 
    println!("name, {}!", name );

    // variables are immutable by default but can be changed using MUT
    let mut age = 26;
    println!("age, {}!", age);
     age = 27;
    println!("updated age, {}!", age);

    // DATA TYPES
    let a: i32 = 16;
    let pi: f64 = 3.14;
    let is_rust_fun: bool = true;
    let letter: char = 'A';
    println!("{}, {}, {}, {}", a, pi, is_rust_fun, letter);

    // FUNCTIONS
    greet("Jethro");
    let result = add(6,4);
    println!("the result is {}!", result);

    // TUPLES AND COMPOUND TYPE
        let person: (&str, i32, bool) = ("Alice", 30, true);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

}

fn greet(bame: &str) {
    println!("hello {}!", bame);
}
 fn add(x:i32, y:i32)->i32{
    x+y
 }