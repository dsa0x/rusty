pub fn run() {
  let stack_i32: i32 = 3;
  let stack_f32 = 3.;

  let stack_i32_2 = stack_i32;

  println!("{:?}", (stack_i32, stack_f32, stack_i32_2));

  let heap_str = String::from("Hello world");
  let heap_str_2 = &heap_str;

  println!("{:?}", (&heap_str, heap_str_2));
}
