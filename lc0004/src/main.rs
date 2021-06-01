
struct Solution{}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut v = Vec::new();
        while let (Some(n1), Some(n2)) = (nums1.get(0), nums2.get(0)) {
            if n1 < n2 {
                v.push(nums1.remove(0));
            } else {
                v.push(nums2.remove(0));
            }
        }
        if nums1.len() != 0 {
            v.append(&mut nums1);
        }
        if nums2.len() != 0 {
            v.append(&mut nums2);
        }
        if len % 2 == 0 {
            ((v.get(len / 2).unwrap() + v.get(len / 2 - 1).unwrap()).clone() as f64) / 2.0
        } else {
            v.get(len / 2).unwrap().clone() as f64
        }
    }
}

fn main() {
    let result = Solution::find_median_sorted_arrays(vec!(1,2,3),vec!());
    println!("{}",result);
}
