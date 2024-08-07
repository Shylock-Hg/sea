struct Solution {}

impl Solution {
    /// 0069. Sqrt(x)
    /// Implement int `sqrt(int x)`.
    /// Compute and return the square root of x, where x is guaranteed to be a
    /// non-negative integer.
    /// Since the return type is an integer, the decimal digits are truncated
    /// and only the integer part of the result is returned.
    fn my_sqrt_search(x: u64, left: u64, right: u64) -> i32 {
        if right <= left {
            return right as i32;
        }
        let mid = left + (right - left) / 2;
        if mid * mid > x {
            Self::my_sqrt_search(x, left, mid)
        } else {
            if (mid + 1) * (mid + 1) > x {
                return mid as i32;
            }
            Self::my_sqrt_search(x, mid + 1, right)
        }
    }
    pub fn my_sqrt(x: i32) -> i32 {
        // sequential serach
        // let mut r = 0;
        // let y : u32 = x as u32;
        // for i in 0..(y + 1) {
        //     if i*i == y {
        //         r = i;
        //         break;
        //     } else if i*i > y {
        //         r = i - 1;
        //         break;
        //     }
        // }
        // return r as i32;

        // binary search
        // return Self::my_sqrt_search(x as u64, 0, x as u64);

        let mut left: u64 = 0;
        let mut right: u64 = x as u64;
        let mut mid: u64 = left + (right - left) / 2;
        while left < right {
            if mid * mid > x as u64 {
                right = mid;
            } else {
                if (mid + 1) * (mid + 1) > x as u64 {
                    return mid as i32;
                }
                left = mid + 1;
            }
            mid = left + (right - left) / 2;
        }
        right as i32
    }

    /// 0540. Single Element in a Sorted Array
    /// You are given a sorted array consisting of only integers where every
    /// element appears exactly *twice*, except for one element which appears
    /// exactly once. Find this single element that appears only once.
    fn single_non_duplicate_(nums: &Vec<i32>, lo: usize, hi: usize) -> usize {
        if lo >= hi {
            return lo;
        }
        let mid = lo + (hi - lo) / 2;
        // key point:
        // make sure the mid is even, so the left and right sequence are counted
        // even too.
        let mid_ = if mid % 2 == 1 { mid - 1 } else { mid };
        if nums[mid_] == nums[mid_ + 1] {
            Self::single_non_duplicate_(nums, mid_ + 2, hi)
        } else {
            Self::single_non_duplicate_(nums, lo, mid_)
        }
    }
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        // let mut lo = 0;
        // let mut hi = nums.len() - 1;
        // let mut mid = lo + (hi - lo) / 2;
        // while lo < hi {
        //     if mid % 2 == 1 {
        //         mid -= 1;
        //     }
        //     if nums[mid] == nums[mid + 1] {
        //         // search right
        //         lo = mid + 2;
        //     } else {
        //         // search left
        //         hi = mid;
        //     }
        //     mid = lo + (hi - lo) / 2;
        // }
        // return nums[mid];

        nums[Self::single_non_duplicate_(&nums, 0, nums.len() - 1)]
    }

    /// 0744. Find Smallest Letter Greater Than Target
    ///  Given a list of sorted characters letters containing only lowercase
    /// letters, and given a target letter target, find the smallest element in
    /// the list that is larger than the given target.
    /// Letters also wrap around. For example, if the target is target = 'z'
    /// and letters = ['a', 'b'], the answer is 'a'.
    fn next_greatest_letter_(
        letters: &Vec<char>,
        target: char,
        lo: usize,
        hi: usize,
    ) -> usize {
        if lo >= hi {
            return hi;
        }
        let mid = lo + (hi - lo) / 2;
        if letters[mid] <= target {
            Self::next_greatest_letter_(letters, target, mid + 1, hi)
        } else {
            Self::next_greatest_letter_(letters, target, lo, mid)
        }
    }
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        // binary search
        // let mut lo: usize = 0;
        // let mut hi: usize = letters.len() - 1;
        // let mut mid: usize = lo + (hi - lo) / 2;
        // while lo < hi {
        //     if letters[mid] <= target {
        //         lo = mid + 1;
        //     } else {
        //         hi = mid;
        //     }
        //     mid = lo + (hi - lo) / 2;
        // }
        // if hi >= letters.len() - 1 && letters[hi] <= target {
        //     return letters[0];
        // }
        // return letters[hi];

        let i =
            Self::next_greatest_letter_(&letters, target, 0, letters.len() - 1);
        if i >= letters.len() - 1 && letters[i] <= target {
            return letters[0];
        }
        letters[i]
    }
}

#[cfg(test)]
mod testing {
    use super::Solution;

    #[test]
    fn test_my_sqrt() {
        let input = 2147483647;
        let output = 46340;
        assert_eq!(output, Solution::my_sqrt(input));
    }

    #[test]
    fn test_single_non_duplicate() {
        let input = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        let output = 2;
        assert_eq!(output, Solution::single_non_duplicate(input));

        let input2 = vec![3, 3, 7, 7, 10, 11, 11];
        let output2 = 10;
        assert_eq!(output2, Solution::single_non_duplicate(input2));
    }

    #[test]
    fn test_next_greatest_letter() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        let output = 'c';
        assert_eq!(output, Solution::next_greatest_letter(letters, target));

        let letters2 = vec!['c', 'f', 'j'];
        let target2 = 'j';
        let output2 = 'c';
        assert_eq!(output2, Solution::next_greatest_letter(letters2, target2));
    }
}
