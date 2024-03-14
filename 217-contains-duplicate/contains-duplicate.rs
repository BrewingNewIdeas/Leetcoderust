use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut check:HashSet<i32>=HashSet::new();
        for i in nums{
            if !check.insert(i){
                return true;
            }
        }
        return false;
    }
}