use std::vec::Vec;

// struct Solution {}
struct Solution {}

impl Solution {
    /// 0167. Two Sum II - Input array is sorted
    /// Given an array of integers that is already sorted in ascending order,
    /// find two numbers such that they add up to a specific target number.
    /// The function twoSum should return indices of the two numbers such that
    /// they add up to the target, where index1 must be less than index2
    ///
    /// Your returned answers (both index1 and index2) are not zero-based.
    /// You may assume that each input would have exactly one solution and you
    /// may not use the same element twice.
    pub fn two_sum_2(numbers: &Vec<i32>, target: i32) -> Vec<i32> {
        // Not use the sorted future
        // Accpted by unsorted array
        // for i in 0..numbers.len() {
        //  let j = i + 1;
        //      for k in j..numbers.len() {
        //          if numbers[i] + numbers[k] == target {
        //              return vec![(i + 1) as i32, (k + 1) as i32];
        //          }
        //      }
        //  }
        // return vec![];

        let mut i = 0 as usize;
        let mut j = numbers.len() - 1;
        while i < j {
            let now = numbers[i] + numbers[j];
            if now == target {
                return vec![(i + 1) as i32, (j + 1) as i32];
            } else if now < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return vec![];
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod testing {
    use super::Solution;
    #[test]
    fn test_two_sum_2() {
        let v = vec![2, 7, 11, 15];
        let i: i32 = 9;
        assert!(
            i == (v[(Solution::two_sum_2(&v, i)[0 as usize] - 1) as usize]
                + v[(Solution::two_sum_2(&v, i)[1 as usize] - 1) as usize])
        );
    }
}
