mod owner;
mod queue;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


fn define_x() -> i32 {
    let x: &str = "This is a string, not an integer";
    x.parse::<i32>().unwrap_or(0) 
}



fn main() {
    println!("Hello, world!");

    let x: i32 = 10;
    let y: i32 = 20;

    {
        let x: i32 = 5; 
        println!("Inner x: {}, y: {}", x, y); 
    }

    for i in 0..5 {
        let x: i32 = i;
        println!("Loop x: {}, y: {}", x, y);
    }

    let mut z: i32 = 15;
    println!("Before mutation: z: {}", z);
    z = 25;
    println!("After mutation: z: {}", z);

    let z = z;
    println!("After shadowing: z: {}", z);

    let dev: &str = "Development";

    println!("Before block: dev: {}", dev);

    {
        let dev: i32 = 30; 
        println!("Inside block: dev: {}", dev); 
    }

    println!("x: {}, y: {}", x, y);

    define_x();

    let sum = add(5, 3);
    let difference = subtract(5, 3);
    let product = multiply(5, 3);
    let quotient = divide(5, 3);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    match quotient {
        Some(q) => println!("Quotient: {}", q),
        None => println!("Cannot divide by zero"),
    }

    let greeting = greet("Alice");
    println!("{}", greeting);


    let (a, b);

    (a,..) = (10, 20);
    [.., b] = [30, 40];

    assert!([a, b] == [10, 40]);
    println!("a: {}, b: {}", a, b);


    let mut test_x: u32 = 5;
    println!("Before mutation: test_x: {}", test_x);
    test_x = 10;
    println!("After mutation: test_x: {}", test_x);

    // Shadowing test
    let test_x: u32 = 15;
    println!("After shadowing: test_x: {}", test_x);


    let myi258: i128 = 1_000_000_000_000_000_000_000;
    println!("c: {}", myi258.to_string());


    let mut sum: i32 = 0;

    for i in -3..6 {
        sum += i;
    }
    println!("Sum from -3 to 9: {}", sum);


    for us in 'a'..='z' {
        println!("Character: {}", us);
    }

    let x = 1_000_000_000_000_000_000_000.01;

   println!("x: {}", x);
   println!("Type of x: {}", type_of(&x));

   let my_name = "Alice";
    println!("myName: {}", my_name);
    println!("Type of myName: {}", type_of(&my_name));


    let mut sum: i32 = 0;
    for i in -3..6 {
        sum += i;
    }
    println!("Sum from -3 to 5: {}", sum);
    assert_eq!(sum, 9);

    let f: bool = true;
    let t: bool = true && false;
    println!("f: {}", f);
    println!("t: {}", t);

    let value: u32 = 5u32;
    let value2 = {
        let x_squard = value * value;
        let x_cubed = x_squard * value;

        x_cubed + x_squard + value
    };


    let z = {
        2 * value
    };

    println!("value: {}, value2: {}, z: {}", value, value2, z);


    owner::owner_ship("Usecase".to_string());

    owner::mutalility();

    owner::box_mutability();

    owner::person_object();

    owner::print_string();

    owner::moveable();

}


fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 1), -1);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_divide() {        
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }
}

