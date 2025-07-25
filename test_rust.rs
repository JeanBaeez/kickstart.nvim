fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled);

    let result = add_numbers(10, 20);
    println!("Result: {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }
}
