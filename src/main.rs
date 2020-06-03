fn main() {
    let opcodes: u16 = 0;
    let a: [u8;4096] = [0;4096];

    let s = String::from("Hello world");
    let hello = &s[0..5];
    first_word(&s);
    println!("{}", hello);
}

fn first_word(s: &String) -> &str {
    return &s[0..5];
}