use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  numbers[2] = 20;
  numbers.push(10);
  numbers.push(10);
  numbers.pop();

  println!("{:?}", numbers);

  println!("Number {}", numbers[0]);

  println!("{}", numbers.len());

  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[0..2];

  println!("{:?}", slice);

  for x in numbers.iter() {
    println!("Number: {}", x);
  }
  println!();
  for x in numbers.iter_mut() {
    *x *= 2;
    println!("Number: {}", x);
  }
}
