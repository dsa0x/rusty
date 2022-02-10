pub fn run() {
  let mut hello = String::from("Hello ");

  hello.push('S');
  hello.push_str("ams");

  println!(
    "{} has len {} and cap of {} and is_empty={}",
    hello,
    hello.len(),
    hello.capacity(),
    hello.is_empty(),
  );

  assert_eq!(10, hello.len());

  println!("{}", hello.replace("Hello", "Hi"));
}
