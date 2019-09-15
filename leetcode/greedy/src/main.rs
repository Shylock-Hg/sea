struct Solution {}

impl Solution {
    /// 0435. Non-overlapping Intervals
    /// Given a collection of intervals, find the minimum number of intervals
    /// you need to remove to make the rest of the intervals non-overlapping.
    /// Note:
    ///   You may assume the interval's end point is always bigger than its
    ///   start point.
    ///   Intervals like [1,2] and [2,3] have borders "touching" but they don't
    ///   overlap each other.
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let mut intervals_mut = intervals.clone();
        intervals_mut.sort_by(|a, b| a[a.len() - 1].cmp(&b[b.len() - 1]));
        let mut count = 1;
        let mut end = intervals_mut[0][intervals_mut[0].len() - 1];
        for i in 1..intervals_mut.len() {
            // find the minimum upper bound
            if intervals_mut[i][0] < end {
                continue;
            }
            count += 1;
            end = intervals_mut[i][intervals_mut[i].len() - 1];
        }
        return intervals_mut.len() as i32 - count;
    }

    /// 0455. Assign Cookies
    /// Assume you are an awesome parent and want to give your children some
    /// cookies. But, you should give each child at most one cookie. Each child
    /// i has a greed factor gi, which is the minimum size of a cookie that the
    /// child will be content with; and each cookie j has a size sj.
    /// If sj >= gi, we can assign the cookie j to the child i, and the child i
    /// will be content. Your goal is to maximize the number of your content
    /// children and output the maximum number.
    /// Note:
    ///   You may assume the greed factor is always positive.
    ///   You cannot assign more than one cookie to one child.
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        // g: children content
        // s: cookie
        // A greedy algorithm is a simple, intuitive algorithm that is used in
        // optimization problems. The algorithm makes the optimal choice at
        // each step as it attempts to find the overall optimal way to solve
        // the entire problem.
        // Why greedy algorithm fit this problem?
        // We assign the cookie to largest child it can content. So this is the
        // best choice for this cookie.
        // Image in the G*S Matrix Solution Space, we choose the child for each
        // cookie, if we choose the one not just fit, maybe there is one cookie
        // just fit the child; So the greedy solution is best.
        // Time: O(M*N), Space: O(M)
        let mut gused = vec![false; g.len()];
        let mut count = 0;
        for si in s {
            let mut now: i32 = -1;
            for gi in 0..g.len() {
                // Find the one for this cookie
                // Greedy find the largest one the cookie can content
                if !gused[gi] {
                    if si >= g[gi] {
                        if now < 0 {
                            now = gi as i32;
                        } else {
                            if g[gi] > g[now as usize] {
                                now = gi as i32;
                            }
                        }
                    }
                }
            }
            if now >= 0 {
                count += 1;
                gused[now as usize] = true;
            }
        }
        return count;

        // Another way to find the cookie for this child
        // Greedy find the minimum one the child can content
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod testing {
    use super::Solution;

    #[test]
    fn test_erase_overlap_intervals() {
        let input = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        let output = 1;
        assert_eq!(output, Solution::erase_overlap_intervals(input));
    }

    #[test]
    fn test_find_content_children() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(1, Solution::find_content_children(g, s));
    }
}
