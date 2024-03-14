use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut Storage1:HashMap<i32,i32>=HashMap::new();
        for i in 0..nums.len(){
            let a=match Storage1.get(&nums[i]){
                Some(&val)=> return vec![val,i as i32],
                _=>Storage1.insert(target-nums[i],i as i32),
            };

        }

        return vec![1,2]
    }
}