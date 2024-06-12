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

// ON utilise la fn string_slice pour les valeurs de type &str.
// ON utilise la fn string pour les valeurs de type String.

fn main() {
    string_slice("blue");  // "blue" est un &str, donc on utilise string_slice.
    string("red".to_string());  // "red".to_string() convertit &str en String.
    string(String::from("hi"));  // String::from("hi") crée un String.
    string("rust is fun!".to_owned());  // to_owned() convertit &str en String.
    string("nice weather".into());  // .into() convertit &str en String.
    string(format!("Interpolation {}", "Station"));  // format! retourne un String.
    string_slice(&String::from("abc")[0..1]);  // &String::from("abc")[0..1] retourne un &str.
    string_slice("  hello there ".trim());  // .trim() retourne un &str.
    string("Happy Monday!".to_string().replace("Mon", "Tues"));  // .to_string().replace(...) retourne un String.
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // .to_lowercase() retourne un String.
}

