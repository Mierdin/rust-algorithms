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

        // We can't change items of a vector while iterating, so let's create
        // a copy of it first, and we'll make changes here while iterating over
        // the immutable original
        let mut vec_mut = vec.to_vec();

        let mut swapped = false;

        let mut i = 0;
        for _item in vec.iter() {
            if i >= vec.len()-1 { break; }

            if vec[i] > vec[i+1] {
                swapped = true;
                // println!("Swapping {} for {}", vec[i], vec[i+1]);

                let value = vec[i];
                vec_mut.remove(i);
                vec_mut.insert(i+1, value);
                break;
            }
            i += 1;
        }

        vec = vec_mut;

        // no swaps? Then we're sorted
        if !swapped { break; }
    }

    vec
}


fn random_vec(size: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        zero_vec.push(rng.gen());
    }
    return zero_vec;
}
