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
            return Self::my_sqrt_search(x, left, mid);
        } else {
            if (mid + 1) * (mid + 1) > x {
                return mid as i32;
            }
            return Self::my_sqrt_search(x, mid + 1, right);
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
        return right as i32;
    }

    /// 0744. Find Smallest Letter Greater Than Target
    ///  Given a list of sorted characters letters containing only lowercase
    /// letters, and given a target letter target, find the smallest element in
    /// the list that is larger than the given target.
    /// Letters also wrap around. For example, if the target is target = 'z'
    /// and letters = ['a', 'b'], the answer is 'a'.
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        // binary search
        let mut lo: usize = 0;
        let mut hi: usize = letters.len() - 1;
        let mut mid: usize = lo + (hi - lo) / 2;
        while lo < hi {
            if letters[mid] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
            mid = lo + (hi - lo) / 2;
        }
        if hi >= letters.len() - 1 && letters[hi] <= target {
            return letters[0];
        }
        return letters[hi];
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
