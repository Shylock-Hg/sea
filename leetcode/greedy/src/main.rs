struct Solution {}

impl Solution {
    /// 0121. Best Time to Buy and Sell Stock
    /// Say you have an array for which the ith element is the price of a given
    /// stock on day i.
    /// If you were only permitted to complete at most one transaction
    /// (i.e., buy one and sell one share of the stock), design an algorithm to
    /// find the maximum profit.
    /// Note that you cannot sell a stock before you buy one.
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Check all choose
        // Time: O(N^2), Space: O(1)
        // let mut profit = 0;
        // for i in 0..prices.len() {
        //     for j in i..prices.len() {
        //         let now = prices[j] - prices[i];
        //         if profit < now {
        //             profit = now;
        //         }
        //     }
        // }
        // return profit;

        // the possible maximum profit just based smaller buy price
        if prices.len() == 0 {
            return 0;
        }
        let mut min = prices.first().unwrap();
        let mut profit = 0;
        for i in 1..prices.len() {
            // find the smaller price to buy
            if min > &prices[i] {
                min = &prices[i];
            // compute the possible maximum profit
            } else {
                profit = std::cmp::max(profit, prices[i] - min);
            }
        }
        return profit;
    }

    /// 0435. Non-overlapping Intervals
    /// Given a collection of intervals, find the minimum number of intervals
    /// you need to remove to make the rest of the intervals non-overlapping.
    /// Note:
    ///   You may assume the interval's end point is always bigger than its
    ///   start point.
    ///   Intervals like [1,2] and [2,3] have borders "touching" but they don't
    ///   overlap each other.
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        // Select interval from smaller upper bound and avoid overlapping
        if intervals.is_empty() {
            return 0;
        }
        let mut intervals_mut = intervals.clone();
        intervals_mut.sort_by(|a, b| a.last().unwrap().cmp(&b.last().unwrap()));
        let mut count = 1;
        let mut end = intervals_mut.first().unwrap().last().unwrap();
        for i in 1..intervals_mut.len() {
            // find the minimum upper bound
            if intervals_mut[i].first().unwrap() < end {
                continue;
            }
            count += 1;
            end = intervals_mut[i].last().unwrap();
        }
        return intervals_mut.len() as i32 - count;
    }

    /// 0452. Minimum Number of Arrows to Burst Balloons
    /// There are a number of spherical balloons spread in two-dimensional
    /// space. For each balloon, provided input is the start and end
    /// coordinates of the horizontal diameter. Since it's horizontal,
    /// y-coordinates don't matter and hence the x-coordinates of start and
    /// end of the diameter suffice. Start is always smaller than end.
    /// There will be at most 104 balloons.
    /// An arrow can be shot up exactly vertically from different points along
    /// the x-axis. A balloon with xstart and xend bursts by an arrow shot at x
    /// if xstart ≤ x ≤ xend. There is no limit to the number of arrows that
    /// can be shot. An arrow once shot keeps travelling up infinitely. The
    /// problem is to find the minimum number of arrows that must be shot to
    /// burst all balloons.
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        // Compute the nonoverlaped range
        // and count the range(balloon)
        if points.is_empty() {
            return 0;
        }
        let mut points_mut = points.clone();
        points_mut.sort_by(|a, b| a.last().unwrap().cmp(&b.last().unwrap()));
        let mut count = 1;
        let mut end = points_mut.first().unwrap().last().unwrap();
        for i in 1..points_mut.len() {
            // find the minimum upper bound
            if points_mut[i].first().unwrap() <= end {
                continue;
            }
            count += 1;
            end = points_mut[i].last().unwrap();
        }
        return count;
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
    fn test_max_profit() {
        let Input = vec![7, 1, 5, 3, 6, 4];
        let Output = 5;
        assert_eq!(Output, Solution::max_profit(Input));
    }

    #[test]
    fn test_erase_overlap_intervals() {
        let input = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        let output = 1;
        assert_eq!(output, Solution::erase_overlap_intervals(input));
    }

    #[test]
    fn test_find_min_arrow_shots() {
        let input = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let output = 2;
        assert_eq!(output, Solution::find_min_arrow_shots(input));
    }

    #[test]
    fn test_find_content_children() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(1, Solution::find_content_children(g, s));
    }
}
