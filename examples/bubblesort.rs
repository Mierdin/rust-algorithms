extern crate rand;

use rand::Rng;
use std::vec::Vec;

fn main() {
    // let values = vec![8, 3, 7, 9, 1, 6, 2, 3];
    let values = random_vec(500);

    println!("Unsorted: {:?}", values);
    let sorted = bubble_sort(values);
    println!("Sorted: {:?}", sorted);
}

fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    loop {
        // Default to false, so we know by the end if we had any swaps,
        // which indicates we need to do another pass
        let mut swapped = false;
        
        // Iterate over the length of vec, swapping any values that are out of order.
        let mut i = 0;
        while i < vec.len() {
            if i >= vec.len()-1 { break; }
            if vec[i] > vec[i+1] {
                swapped = true;
                let value = vec[i];
                vec.remove(i);
                vec.insert(i+1, value);
                break;
            }
            i += 1;
        }

        // no swaps? Then we're sorted, and we can exit and return the final `vec`
        if !swapped { break; }
    }

    vec
}


fn random_vec(size: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        zero_vec.push(rng.gen_range(0, 1000));
    }
    return zero_vec;
}
