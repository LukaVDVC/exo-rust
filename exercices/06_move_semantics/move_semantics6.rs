// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
let data = "Rust is great!".to_string();
let last_char = get_char(data.clone());
println!("Last character: {}", last_char);
let mut data_clone = data.clone();
string_uppercase(&mut data_clone);
println!("Uppercase string: {}", data_clone);
}
// Should not take ownership
fn get_char(data: String) -> char {
data.chars().last().unwrap()
}
// Should take ownership
fn string_uppercase(data: &mut String) {
*data = data.to_uppercase();
println!("{}", data);
}
