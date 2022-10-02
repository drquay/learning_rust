fn main() {
    use std::fs::File;
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(_file) => println!("Hello file"),
        Err(_err) => println!("File is not existing")
    };
}
