// three type od structures in RUS
// 1. struct      2. tuple       3. unitlike


struct Person {
    name: String,
    age: u8,
}

struct Color(i32, i32, i32);

struct unitLike;

// methods on struct
struct Sqaure{
    width:u32,
    height:u32,
}

impl Sqaure{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn resize(&mut self, width:u32,height:u32){
        self.width = width;
        self.height = height;
    }
}

fn lifetimeInRust<'a>(x:&'a i16)-> &'a i16{
    x
}


fn main() {
    println!("Hello, world!");

    let mut person = Person {
        name: String::from("John"),
        age: 30,
    };
    person.name = String::from("Bob");
    println!("{} is {}", person.name, person.age);

    let red = Color(255, 0, 0);
    println!("{} {} {}", red.0, red.1, red.2);

    let mut small_sqaure = Sqaure{width:10,height:10};
    println!("{}", small_sqaure.area());

    small_sqaure.resize(20,20);
    println!("{}", small_sqaure.area());

    let x = 10;
    println!("{}", lifetimeInRust( &x));
}
