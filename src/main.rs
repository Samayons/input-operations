mod get_string;

fn main() {
    let name = get_string::get_input("What is your name? ");
    println!("Hello {}", name);
}
