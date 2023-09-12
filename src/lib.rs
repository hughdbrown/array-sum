use std::cmp::Ordering;

pub fn array_sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn array_sum_lookup(nums: &[i32]) -> Vec<i32> {
    let mut fast: Vec<i32> = vec![];
    let mut prev_sum = 0;
    for num in nums.iter() {
        fast.push(prev_sum);
        prev_sum += num;
    }
    fast.push(prev_sum);
    fast
}

pub fn find_subarray(nums: &[i32], sum: i32) -> Option<(usize, usize)> {
    for len in (1..nums.len() + 1).rev() {
        for start in 0..nums.len() - len + 1 {
            if array_sum(&nums[start..start+len]) == sum {
                return Some((start, len));
            }
        }
    }
    None
}

pub fn find_subarray_smart(nums: &[i32], sum: i32) -> Option<(usize, usize)> {
    let fast = array_sum_lookup(nums);
    let fast_len: usize = fast.len();

    let mut i = 0usize;
    let mut j = 0usize;
    while j < fast_len {
        let tmp = fast[j] - fast[i];
        match tmp.cmp(&sum) {
            Ordering::Less => j += 1,
            Ordering::Greater => i += 1,
            Ordering::Equal => { return Some((i, j - i)); },
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works() {
        let a = vec![2, 4, 3, 6, 7, 1, 8, 9, 5];
        let b = array_sum_lookup(&a);
        assert!(b.len() == a.len() + 1);
        for i in 0..a.len() {
            for j in i+1..a.len() {
                assert!(j - i >= 1);
                let x: &[i32] = &a[i..j];
                assert!(! x.is_empty());
                let target: i32 = x.iter().sum();
                let target2: i32 = b[j] - b[i];
                assert_eq!(target, target2);
            }
        }
    }
}
