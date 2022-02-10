pub fn run() {
  greeting("Hello", "Sams");
  let sum = add(20, 100);
  println!("{:?}", sum);

  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("{:?}", add_nums(10, 20));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  return n1 + n2;
}
