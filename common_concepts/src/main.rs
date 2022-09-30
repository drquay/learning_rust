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
