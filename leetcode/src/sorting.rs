use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

struct Solution {}

impl Solution {
    /// Quick sort
    fn pivot2(nums: &mut [i32], lo: isize, hi: isize) -> isize {
        let pivot = nums[hi as usize];
        let mut p = lo;
        for i in lo..hi {
            if nums[i as usize] < pivot {
                nums.swap(i as usize, p as usize);
                p += 1;
            }
        }
        nums.swap(p as usize, hi as usize);
        p
    }
    fn quick_sort_helper(nums: &mut [i32], lo: isize, hi: isize) {
        if lo <= hi {
            let p = Self::pivot2(nums, lo, hi);
            Self::quick_sort_helper(nums, lo, p - 1);
            Self::quick_sort_helper(nums, p + 1, hi);
        }
    }
    pub fn quick_sort(nums: &mut [i32]) {
        Self::quick_sort_helper(nums, 0, (nums.len() - 1) as isize);
    }

    /// Merge Sort
    fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut index = 0_usize;

        while left < arr1.len() && right < arr2.len() {
            if arr1[left] < arr2[right] {
                ret[index] = arr1[left];
                left += 1;
            } else {
                ret[index] = arr2[right];
                right += 1;
            }
            index += 1;
        }
        if left < arr1.len() {
            ret[index..].copy_from_slice(&arr1[left..])
        } else {
            ret[index..].copy_from_slice(&arr2[right..])
        }
    }
    pub fn merge_sort(arr: &mut [i32]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        Self::merge_sort(&mut arr[..mid]);
        Self::merge_sort(&mut arr[mid..]);

        let mut ret = arr.to_vec();

        Self::merge(&arr[..mid], &arr[mid..], &mut ret);

        arr.copy_from_slice(&ret);
    }

    /// 0075. Sort Colors
    /// Given an array with n objects colored red, white or blue, sort them
    /// in-place so that objects of the same color are adjacent, with the
    /// colors in the order red, white and blue.
    /// Here, we will use the integers 0, 1, and 2 to represent the color red,
    /// white, and blue respectively.
    /// Note: You are not suppose to use the library's sort function for this
    /// problem.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // index: coloer, value: count
        let mut color: [usize; 3] = [0; 3];
        let mut index = 0;
        for i in &mut nums.iter() {
            color[*i as usize] += 1;
        }
        for i in 0..color.len() {
            for j in 0..color[i] {
                nums[index] = i as i32;
                index += 1;
            }
        }
    }

    /// 0215. Kth Largest Element in an Array
    /// Find the kth largest element in an unsorted array. Note that it is the
    /// kth largest element in the sorted order, not the kth distinct element.
    ///   You may assume k is always valid, 1 ≤ k ≤ array's length.
    fn pivot(v: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
        // Make the pivot seperate the sequence
        // The left less than pivot
        // The right bigger than pivot
        // let p = hi;
        // let mut i = lo;
        // for j in lo..hi {
        //     if v[j] < v[p] {
        //         v.swap(j, i);
        //         i += 1;
        //     }
        // }
        // v.swap(i, p);
        // return i;

        let mut i = lo as i32;
        let mut j = hi as i32 + 1;
        loop {
            loop {
                i += 1;
                if v[i as usize] >= v[lo] || i >= hi as i32 {
                    break;
                }
            }
            loop {
                j -= 1;
                if v[j as usize] <= v[lo] || j <= lo as i32 {
                    break;
                }
            }
            if i >= j {
                break;
            }
            v.swap(i as usize, j as usize);
        }
        v.swap(lo, j as usize);
        j as usize
    }
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        assert!((k as usize) <= nums.len());
        // Sort first!
        // Time: O(NlogN), Space: O(1)
        // let mut nums_mut = nums.clone();
        // nums_mut.sort();
        // return nums_mut[nums_mut.len() - k as usize];

        // BinaryHeap - Min Heap
        // Time: O(NlogK), Space: O(K)
        // let mut bp = BinaryHeap::new();
        // for i in nums {
        //     bp.push(Reverse(i)); // Make Min Heap
        //     if bp.len() > k as usize {
        //         bp.pop();
        //     }
        // }
        // return bp.peek().unwrap().0;

        // Partition like in Quick Soring
        // Time: O(N^2), Space: O(1)
        // If the pivot less than K, that the K largest element in subsequence
        // bigger than the pivot, otherwise the opposite. So try partition
        // again in the subsequence contians K largest element.
        // Exit when the pivot is Just the K
        // Better to shuffle input!
        let mut nums_mut = nums.clone();
        let ku = nums_mut.len() - k as usize;
        let mut i = 0;
        let mut j = nums_mut.len() - 1;
        while i < j {
            let p = Solution::pivot(&mut nums_mut, i, j);
            if p == ku {
                break;
            } else if p < ku {
                i = p + 1;
            } else {
                j = p - 1;
            }
        }
        nums_mut[ku]
    }

    /// 347. Top K Frequent Elements
    /// Given a non-empty array of integers, return the k most frequent
    /// elements.
    /// You may assume k is always valid, 1 ≤ k ≤ number of unique elements.
    /// Your algorithm's time complexity must be better than O(n log n),
    /// where n is the array's size.
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // (num, freq)
        let mut freq_map: HashMap<i32, i32> =
            HashMap::with_capacity(nums.len());
        for n in &nums {
            freq_map.insert(
                *n,
                if let Some(v) = freq_map.get(n) {
                    v + 1
                } else {
                    1
                },
            );
        }
        let mut bucket: Vec<LinkedList<i32>> =
            vec![LinkedList::new(); nums.len() + 1];
        for (k, v) in freq_map {
            bucket[v as usize].push_back(k);
        }
        let mut r: Vec<i32> = Vec::with_capacity(k as usize);
        let mut counter = 0;
        for i in bucket.iter().rev() {
            for j in i {
                r.push(*j);
                counter += 1;
                if counter >= k {
                    break;
                }
            }
            if counter >= k {
                break;
            }
        }
        r
    }

    /// 0451. Sort Characters By Frequency
    /// Given a string, sort it in decreasing order based on the frequency of
    /// characters.
    pub fn frequency_sort(s: String) -> String {
        let mut r = String::with_capacity(s.len());
        // index: character, value: frequency
        let mut alphabet: [usize; 256] = [0; 256];
        // index: frequency, value: character
        let mut freq: Vec<LinkedList<u8>> =
            vec![LinkedList::new(); s.len() + 1];
        let bytes = s.as_bytes();
        for c in bytes {
            alphabet[*c as usize] += 1;
        }
        for i in 0..alphabet.len() {
            freq[alphabet[i]].push_back(i as u8);
        }
        for i in (0..freq.len()).rev() {
            for c in &freq[i] {
                for _ in 0..i {
                    r.push(*c as char);
                }
            }
        }
        r
    }
}

#[cfg(test)]
mod testing {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_sort_colors() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn test_find_kth_largest() {
        //        let k = 4;
        //        let input = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        //        let target = 4;
        let k = 2;
        let input = vec![7, 6, 5, 4, 3, 2, 1];
        let target = 6;
        assert!(target == Solution::find_kth_largest(input, k));
    }

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 2];
        let k = 2;
        let mut r = HashSet::new();
        r.insert(1);
        r.insert(2);
        let mut e = HashSet::new();
        let v = Solution::top_k_frequent(nums, k);
        for i in v {
            e.insert(i);
        }
        assert_eq!(r, e);
    }

    #[test]
    fn test_quick_sort() {
        let mut nums = vec![2, 1, 5, 7, 3, 2, 2, 0];
        let expected = [0, 1, 2, 2, 2, 3, 5, 7];
        Solution::quick_sort(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_merge_sort() {
        let mut nums = vec![2, 1, 5, 7, 3, 2, 2, 0];
        let expected = [0, 1, 2, 2, 2, 3, 5, 7];
        Solution::merge_sort(&mut nums);
        assert_eq!(nums, expected);
    }
}
