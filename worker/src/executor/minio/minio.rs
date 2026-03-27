use std::env;

fn main() {
  // Get a specific environment variable
  match env::var("HOME") {
      Ok(val) => println!("HOME: {}", val),
      Err(e) => println!("Couldn't read HOME: {}", e),
  }
}