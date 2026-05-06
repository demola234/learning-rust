pub fn owner_ship(fold: String) -> String {
 let s = String::from(fold);
 let s2 = s;

 println!("{}", s2);

 return s2;
}

pub fn mutalility() {
    let s: String = String::from("hello, ");

    let mut s1 = s;

    s1.push_str("world");


    println!("{}", s1);
}

pub fn box_mutability() {
 let x: Box<i32> = Box::new(5);

 let mut y: Box<i32> = Box::new(3);

 *y = 4;

 assert_eq!(*x, 5);
}

pub fn person_object() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }


    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person {name ,ref   age} = person;

    println!("The person name is {}, their age is {}", name, age);
}


pub fn print_string() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s = t.0;

    println!("{}", _s);
}

pub fn moveable() {
    let s: String = String::from("hello");

    borrow_object(&s);
}

fn borrow_object(_s: &String) {}