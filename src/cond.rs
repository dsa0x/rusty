pub fn run() {
  let age = 22;
  let check_id: bool = true;
  let of_age: bool = true;

  if age > 21 && check_id || of_age {
    println!("Bartender: drink")
  } else if age <= 21 {
    println!("No drink")
  } else {
    println!("ID")
  }

  let is_age = if age >= 21 { true } else { false };
  println!("Age is {}", is_age)
}
