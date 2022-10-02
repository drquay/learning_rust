fn main() {
    shadow_variable();
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    let mut s = String::from("hello");

    let r1 = &s; 
    let r2 = &s; 
    let r3 = &mut s; 

    println!("r3: {}", *r3);

    struct User<'a> {
        active: bool,
        username: &'a str,
        email: &'a str,
        sign_in_count: u64,
    }

    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

fn shadow_variable() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inside block: {}", x);
    }
    println!("outside block: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces len: {}", spaces);
}
