fn main() {
    fmt_println();
    fmt_debug();
}

fn fmt_debug() {

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    println!("{:?} month in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Hau", "Lam", actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));

    let name = "Liam";
    let age = 32;
    let liam = Person{name, age};
    println!("{:#?}", liam);
}

fn fmt_println() {

    println!("{} days", 31);
    
    println!("{0}, this is {1}. {1} this is {0}", "Hau", "Lam");
    
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    
    println!("Base 10 repr:                  {}", 69420);
    println!("Base 2 (binary) repr:          {:b}", 69420);
    println!("Base 8 (octal) repr:           {:o}", 69420);
    println!("Base 16 (hexadecimal) repr:    {:x}", 69420);
    println!("Base 16 (hexadecimal) repr:    {:X}", 69420);

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0>width$}", number=1, width=10);

    println!("My name is {0}, {1} {0}", "Hau", "Lam");

    let number: f64 = 1.0;
    let width: usize = 20;
    println!("{number:0>width$}");

    println!("Pi is roughtly {:.3}", std::f64::consts::PI);

    // #[allow(dead_code)]
    // struct Structure(i32);

    // let str: &Structure = &Structure(3);
    // println!("This struct {:?} won't print...", Structure(3));
}