impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {

        let mut holdSquares:Vec<i32>=nums.iter()
        .map(|&x| x.pow(2))
        .collect();
        
        holdSquares.sort();
        return holdSquares;
            


        
        
    }
}