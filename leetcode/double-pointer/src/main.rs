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
        // Not use the sorted future,
        // so search the half of Solution Space(Marix)
        // Accpted by unsorted array
        // The O(N^2) solution, (N-1 + ... + 1) ~ (N-1)(N/2) ~ O(N^2)
        // for i in 0..numbers.len() {
        //  let j = i + 1;
        //      for k in j..numbers.len() {
        //          if numbers[i] + numbers[k] == target {
        //              return vec![(i + 1) as i32, (k + 1) as i32];
        //          }
        //      }
        //  }
        // return vec![];

        // For those of you who are wondering how this works,
        //   here is a quick explanation:
        // Each sum is characterized by two indices (i, j),
        //   where 0 <= i < j < n with n the length of the input array.
        //   If we were to compute them explicitly, we end up with an
        //   n-by-n matrix.
        // If the input array is not sorted, to search for the target,
        //   there is no good way but comparing it with elements from the above
        //   matrix one by one. This is the naive O(n^2) solution.
        //   Of course you can use a HashMap to memorize visited elements and
        //   cut down the time to O(n) so we have the classic space-time
        //   tradeoff.
        // Now if the input array is sorted, the n-by-n summation matrix will
        //   have the following properties:
        //     Integers in each row are sorted in ascending order from left to
        //       right.
        //     Integers in each column are sorted in ascending order from top
        //       to bottom.
        // To find the target, we do not have to scan the whole matrix now
        //   since it exhibits some partial order. We may start from the
        //   top-right (or bottom-left) corner, then proceed to the next row or
        //   previous column depending on the relationship between the matrix
        //   element and the target until either it is found or all rows and
        //   columns are exhausted. The key here is that we can get rid of a
        //   whole row or column due to the two properties of the matrix
        //   specified above.
        // This procedure just like search in BST(Binary Search Tree)
        // If you have finished leetcode problem "240. Search a 2D Matrix II",
        //   you will find that this is exactly the same problem, except now of
        //   the two indices, the first has to be smaller than the second.
        //   Time complexity for "leetcode 240" is O(m + n), while for this
        //   problem we have m = n, plus the indices constraint so the time
        //   complexity will be O(n). Also we do not need the HashMap now so
        //   space complexity will be O(1).
        // Refer to https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/51239/Share-my-java-AC-solution./51905
        // The O(N) solution
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

    /// 0633. Sum of Square Numbers
    /// Given a non-negative integer c, your task is to decide whether there're
    /// two integers a and b such that a2 + b2 = c.
    pub fn judge_square_sum(c: i32) -> bool {
        // This is same as #0167 in algorithm level
        // Search in 0~N Marix Solution Space, BST search method
        // O(N)
        // assert!(c >= 0);
        // let mut i = 0;
        // let mut j = c;
        // while i <= j {
        //     let r = i*i + j*j;
        //     if r == c {
        //          return true;
        //     } else if r < c {
        //         i += 1;
        //     } else {
        //         j -= 1;
        //     }
        // }
        // return false;

        // Search in 0~SQRT(N) Matrix Solution Space, BST search method
        // O(SQRT(N))
        assert!(c >= 0);
        let mut i = 0;
        let mut j = (c as f32).sqrt() as i32;
        while i <= j {
            let r = i * i + j * j;
            if r == c {
                return true;
            } else if r < c {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return false;
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

    #[test]
    fn test_judge_square_sum() {
        let num = 5;
        assert!(true == Solution::judge_square_sum(num));
    }
}
