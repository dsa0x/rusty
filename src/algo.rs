use std::collections::HashMap;

pub fn run() {
  println!("{:?}", two_sum(vec![-1, -2, -3, -4, -5], -8));
  println!("{:?}", two_sum(vec![2, 7, 10, 12], 9));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map: HashMap<i32, i32> = HashMap::new();
  for (i, el) in nums.iter().enumerate() {
    let compl = target - el;
    let idx = i32::try_from(i).ok().unwrap();
    if map.contains_key(el) {
      let val = map.get(el).unwrap();
      return vec![idx, *val];
    }
    map.insert(compl, idx);
  }
  Vec::new()
}
