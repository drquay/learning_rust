fn main() {
    vector();
    string();
    map();
}

fn map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("A"), 10);
    scores.insert(String::from("A"), 20);
    scores.insert(String::from("B"), 100);

    for (k, v) in &scores {
        println!("key: {}, value: {}", k, v);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);

    let t = "नमस्ते";
    for c in t.chars() {
        println!("{}", c);
    }
}

fn vector() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(8);
    match third {
        Some(ele) => println!("The nine element is {}", ele),
        None => println!("There is no nine element")
    }

    println!("Iterator vector");
    let v1 = vec![1,100,32,57];
    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("{}", i);
    }
}