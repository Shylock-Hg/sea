use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

struct Solution {}

impl Solution {
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
        return j as usize;
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
        return nums_mut[ku];
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
        return r;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod testing {
    use super::Solution;

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
        let r = vec![1, 2];
        assert_eq!(r, Solution::top_k_frequent(nums, k));
    }
}
