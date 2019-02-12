mod parser;

use std::io;

fn main() {
  println!("Please enter an RGB color.");

  let mut maybe_color = String::new();

  io::stdin()
    .read_line(&mut maybe_color)
    .expect("Failed to read line.");

  let mut maybe_color = maybe_color.trim().to_ascii_lowercase();

  match maybe_color.len() {
    0 => println!("Sorry, you can't enter an empty string."),
    _ => println!("Thanks for not entering an empty string!"),
  }

  println!("You picked: {}", maybe_color);

  match parser::parse_rgb(maybe_color.as_mut_str()) {
    Ok(res) => println!("{:?}", res),
    Err(message) => println!("{}", message),
  }
}
