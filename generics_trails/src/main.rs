fn main() {
    generics();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Say hello: {}", article.hello());
    println!("ToString: {}", article.to_string());

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_string(&string1, &string2);
        println!("Longest string is: {}", result);
    }
}

fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

pub trait ToString {
    fn to_string(&self) -> String {
        String::from("This is to string....")
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn hello(&self) -> String {
        String::from("(Hello...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

impl ToString for NewsArticle {

}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self)->String{
        format!("{}: {}", self.username, self.content)
    }
}

fn generics() {
    let numbers = vec![10, 2, 3, 5, 20];
    let mut greates = &numbers[0];

    for i in &numbers {
        if greates < i {
            greates = i;
        }
    }
    println!("1. greatest number is {}", greates);
    // println!("2. greatest number is {}", find_largest_number(&numbers));
}

// fn find_largest_number<T>(numbers: &[T]) -> &T {
//     let mut largest = &numbers[0];
//     for number in numbers {
//         if largest < number {
//             largest = number;
//         }
//     }
//     largest
// }