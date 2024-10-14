struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();

        for i in 0 .. n -1 {
            for j in i + 1 .. n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
