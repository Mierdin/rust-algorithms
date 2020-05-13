extern crate rand;

use rand::Rng;
use std::vec::Vec;

fn main() {

    // let values = vec![8, 3, 7, 9, 1, 6, 2, 3, 5];
    let values = random_vec(500);

    println!("Unsorted: {:?}", values);
    let sorted = merge_sort(values);
    println!("Sorted: {:?}", sorted);
}

fn merge_sort(vec: Vec<i32>) -> Vec<i32> {

  // If we're already down to the atomic level, return right away
  if vec.len() == 1 {
    return vec;
  }

  let d = vec.len() / 2;
  let v1 = vec[0..d].to_vec();
  let v2 = vec[d..].to_vec();

  let v1 = merge_sort(v1);
  let v2 = merge_sort(v2);

  // We know how big the returned vector needs to be, so lets pre-allocate it
  // all at once and set everything to zeros (we'll overwrite these below)
  let size = v1.len() + v2.len();
  let mut retvec = vec![0; size];

  let mut i = 0;  // Counter for v1
  let mut j = 0;  // Counter for v2
  let mut k = 0;  // Counter for retvec

  // Step through v1 and v2, adding the smallest values from between them to retvec
  while i < v1.len() && j < v2.len() {
    if v1[i] < v2[j] {
      retvec[k] = v1[i];
      i+=1;
    } else {
      retvec[k] = v2[j];
      j+=1;
    }
    k+=1;
  }

  // Add all remaining values.
  while i < v1.len() {
    retvec[k] = v1[i];
    i+=1;
    k+=1;
  }
  while j < v2.len() {
    retvec[k] = v2[j];
    j+=1;
    k+=1;
  }

  retvec
}


fn random_vec(size: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        zero_vec.push(rng.gen_range(0, 1000));
    }
    return zero_vec;
}
