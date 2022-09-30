fn main() {
    fmt_println();
    println!("==================================");
    fmt_debug();
    println!("==================================");
    fmt_display();
}

fn fmt_display() {
    use std::fmt;
    
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D{
        x: f64,
        y: f64
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {:?}, y: {:?}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}", small = small_range, big = big_range);

    let point = Point2D{x: 3.3, y: 7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("What does Point2D look like in binary: {:b}?", point);

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {write!(f, ", ")?;}
                write!(f, "{}: {}", count, v)?;
            }
            write!(f,"]")
        }
    }

    let v = List(vec![1,2,3]);
    println!("{}", v);


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