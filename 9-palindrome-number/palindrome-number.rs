impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0{
            return false
        }
        let mut tempclon:i32=x;
        let mut recon:i32=0;
        while tempclon>0{
            recon*=10;
            recon+=tempclon%10;
            tempclon/=10;

        }
        if recon==x{
            return true
        }
        return false
        
    }
}