use rand::Rng;
use std::time::Instant;

use array_sum::{array_sum, find_subarray, find_subarray_smart};

fn sanity_check(nums: &[i32], target: i32, result: (usize, usize)) {
    // Sanity check: make sure reported range has target value
    let (s, t) = result;
    assert_eq!(target, nums[s..s+t].iter().sum());
}

fn main() {
    let mut nums = vec![];
    let mut rng = rand::thread_rng();
    let count: usize = 1000;
    for _ in 0..count {
        let num = rng.gen_range(0..10);
        nums.push(num);
    }

    for trial in 0..100 {
        let i: usize = rng.gen_range(0..count/2);
        let j: usize = rng.gen_range(i+1..count); 
        let target_values = &nums[i..=j];
        let target = array_sum(target_values);

        println!("---------------- {trial}");
        println!("target is {:?} in range ({:?}, {:?})", target, i, j);

        let start = Instant::now();
        let result1 = find_subarray(&nums, target);
        let duration1 = start.elapsed();

        let start = Instant::now();
        let result2 = find_subarray_smart(&nums, target);
        let duration2 = start.elapsed();

        match (result1, result2) {
            (Some(result1), Some(result2)) => {
                println!("Naive: {:?} {:?}", result1, duration1);
                println!("Smart: {:?} {:?}", result2, duration2);

                sanity_check(&nums, target, result1);
                sanity_check(&nums, target, result2);
            },
            (_, _) => {
                println!("Error");
            },
        }
    }
}

