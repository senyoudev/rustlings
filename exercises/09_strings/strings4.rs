// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // This is a string literal so it is a &str
    string("red".to_string()); // This is a string because we change it to a String using to_string()
    string(String::from("hi")); // This is a string because we change it to a String using from()
    string("rust is fun!".to_owned()); // This is a string because we change it to a String using to_owned(), it is like we clone the string
    string("nice weather".into()); // This is a string because we change it to a String using into()
    string(format!("Interpolation {}", "Station")); // This is a string because we change it to a String using format! macro
    string_slice(&String::from("abc")[0..1]); // This is a string slice because we take a slice of the string
    string_slice("  hello there ".trim()); // trim() returns a string slice
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
