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

    let personn: (&str, f64, bool) = ("jethro", 12.6, false);
    println!("Nme: {}", personn.0);
    println!("height: {}", personn.1);
    println!("is ugly: {}", personn.2);

    // Destructuring tuples
    let coordinates = (4,7);
    let (x,y) = coordinates;
    println!("x {}, y {}",x, y);

    // OWNERSHIP, BORROWING AND REFERENCE
    let my_name = String::from ("nenkang");
    let new_name = my_name;
    println!("new name {}", new_name);

    // Borrowing with references
    let nme = String:: from("jaytee");
    greeet(&nme);
    println!("back in main{}", nme);

    // comparing string and string literal
    let s_literal: &str = "hello";
    let s_string: String = String::from("hello");
    println!("literal {}", s_literal);
    println!("string{}", s_string);
// example
    let greeting:&str= "Hi there!";
    let mut dynamic_greeting: String = String::from("Hi there!");

    // Modify the dynamic greeting
    dynamic_greeting.push_str(" Welcome!");

    println!("Static: {}", greeting);
    println!("Dynamic: {}", dynamic_greeting);

    // MATCH STATEMENTS AND EXAMPLES
    // EXAMPLE1(match with numbers)
    let number = 6;
    match number {
      1 => println!("one!"),
      2 => println!("two!"),
      3 => println!("three!"),
      _=> println!("something else!"),

    }

    // Example2(match with strings)
    let command = "paused";
    match command {
      "start"=> println!("Game started!"),
      "pause"=> println!("Game paused!"),
      "quit"=> println!("exiting game...!"),
      _=> println!("unknown command!"),

    }

    // example 3 (match with range and binding)
    let age = 49;

    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=59 => println!("Adult"),
        _ => println!("Senior"),
    }

    // example  4
    let day = "sunday";
    
    match day {
      "monday" => println!("start of new work week"),
      "sunday" => println!("it is weekend"),
      _ => println!("other days of the week"),
    }

    // OPTION TYPE <>
    // Example1 ()
    let present =Some("babseball kit");
    let absent: Option<&str> = None;

    match present {
     Some(gift) => println!("You got a {}!", gift),
      None => println!("sorry nothing at the moment"),
    }

     match absent {
      Some(gift) => println!("you got a new {}", gift),
      None => println!("sorry nothing at the moment"),
    }

    // Example 2 (improved real-world example i.e login system)
    let username = get_username(1);

    match username {
      Some(suna) => println!("welcome {}!", suna),
      None => println!("username not found"),

    }

    let email: Option<&str> = Some("jayteemonie@gmail.com");
    let phone: Option<&str> = None;

    check_contact(email);
    check_contact(phone);

    // example3
        let names = ["Alice", "Bob", "Charlie"];

    let index = find_index("Bob", &names);
    match index {
        Some(i) => println!("Found at index: {}", i),
        None => println!("Name not found"),
    }

    let missing = find_index("David", &names);
    println!("Result: {:?}", missing);



}

fn greet(bame: &str) {
    println!("hello {}!", bame);
}
 fn add(x:i32, y:i32)->i32{
    x+y
 }
  fn greeet(person:&String){
    println!("hello {}", person)

  }
  // OPTION TYPE
  fn get_username (id:u32) -> Option<&'static str> {
   if id == 1 {
    Some("footballfans88")
   }
   else {
    None
   }
  }
  fn check_contact (contact: Option<&str>){
    match contact {
      Some(info) => println!("contact found : {}", info),
      None => println!("No contact provided"),
    }
  }

  fn find_index(name: &str, list: &[&str]) -> Option<usize> {
    for (index, &item) in list.iter().enumerate() {
        if item == name {
            return Some(index);
        }
    }
    None
}