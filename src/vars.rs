pub fn run() {
  let name = "Sams";
  let mut age = 26;
  println!("My name is {} and I am {}", name, age);
  age = 25;
  println!("My name is {} and I am {}", name, age);

  //consts
  const ID: i32 = 001;
  println!("ID: {}", ID);

  //multivars
  let (myname, myage) = ("Sams", 25);
  println!("{} {}", myname, myage)
}
