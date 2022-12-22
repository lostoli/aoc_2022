use std::{fs::File, io::Read};

fn main() {
  let mut file = File::open("input").expect("Failed to open input"); // open file - panic if not exist
  let mut input = String::new();

  file.read_to_string(&mut input).expect("Can't read String"); // if the string isn't there for some reason

  let orig = input.trim_end()
    .split("\n")
    .map(|n| n.parse::<isize>().unwrap())
    .map(|k| k * 811_589_153_isize)
    .collect::<Vec<isize>>();
  let mut mixing = (0..orig.len()).collect::<Vec<usize>>();

  for _ in 0..10 {
    for n in 0..orig.len() {
      let ixof: usize = mixing.iter().position(|&itm| itm == n).unwrap();
      let number: isize = orig[n]; // number means the number
      assert_eq!(mixing.remove(ixof), n); // if the removed value isn't what we expect, error
      mixing.insert((ixof as isize + number - 1).rem_euclid((orig.len() as isize) - 1_isize) as usize + 1, n);
    }
  }

  // apply the mixing
  let mixed = mixing.iter().map(|index| orig[*index]).collect::<Vec<isize>>();
  let index_of_zero = mixed.iter().position(|&mx| mx == 0).unwrap();
  let coords = vec![
    mixed[(index_of_zero + 1000).rem_euclid(mixed.len())],
    mixed[(index_of_zero + 2000).rem_euclid(mixed.len())],
    mixed[(index_of_zero + 3000).rem_euclid(mixed.len())],
  ];

  println!("sum of all three coords: {}", coords.iter().sum::<isize>());
}
