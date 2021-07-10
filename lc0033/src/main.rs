struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            let mid = (right - left) / 2 + left;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < nums[right] { // 右边有序
                if nums[mid] < target && target <= nums[right] {
                    // 到右边找
                    left = mid + 1;
                } else {
                    if mid == 0 {
                        return - 1;
                    }
                    right = mid - 1;
                }
            } else {
                if nums[left] <= target && target < nums[mid] {
                    if mid == 0 {
                        return -1;
                    }
                    // 到左边找
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        -1
    }
}
fn main() {
    let result = Solution::search(vec![ 4, 5, 6, 7, 0, 1, 2,], 0);
    println!("{}", result);
}
