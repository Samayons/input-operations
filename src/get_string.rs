use std::io;

pub fn get_input(string_placeholder: &str) -> String {
  println!("{}", string_placeholder);

  let mut string = String::new();

  io::stdin()
    .read_line(&mut string)
    .expect("Failed to read line, expected a string");

  return string;
}
