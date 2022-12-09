use std::fs::File;
use std::io::Write;

fn write(name: std::string::String) {
    let mut f = File::create(name).unwrap();
    writeln!(&mut f, "This is a test").unwrap();
}

fn main() {
    println!("Hello rust world!");

    write(String::from("test.txt"));
    println!("Wrote file");

    std::fs::copy("test.txt", "test-2.txt").unwrap();
    println!("Copied file");
}