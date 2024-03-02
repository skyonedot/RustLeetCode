use std::collections::HashMap;
fn main() {
    let vec: Vec<i32> = vec![1, 2, 6, 3];
    let result = Solution::two_sum(vec, 9);
    println!("{:?}", result);
    // println!("{:?}",vec)
    // println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut return_vec: Vec<i32> = vec![];
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if (nums[i] + nums[j] == target) {
                    return_vec.push(i as i32);
                    return_vec.push(j as i32);
                }
            }
        }
        return return_vec;
    }

    pub fn two_sum_better_a(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_index = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if num_index.contains_key(&complement) {
                return vec![*num_index.get(&complement).unwrap() as i32, i as i32];
            }
            num_index.insert(num, i);
        }

        vec![]
    }
}
