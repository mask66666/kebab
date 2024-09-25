#[allow(non_snake_case)]
fn HelloWorld(arg: &str) {
  if arg.to_lowercase() == "println!" {
    println!("{}", arg);
  } else {
    panic!();
  }
}

fn main() {
  HelloWorld("println!");
}