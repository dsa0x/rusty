pub fn run() {
  let person: (&str, &str, i8) = ("SAMS", "Mars", 25);

  println!("{} is from {} and {}", person.0, person.1, person.2)
}
