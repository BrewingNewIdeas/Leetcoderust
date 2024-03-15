use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false
        }
        let mut LetStores:HashMap<char, i32>=HashMap::new();
        for (i,v) in s.chars().zip(t.chars()){
           *LetStores.entry(i).or_insert(0)+=1;
           *LetStores.entry(v).or_insert(0)-=1;

        }
        for (c,d) in LetStores.drain(){
            if d!=0{
                return false;
            }
        }
        return true;
        
    }


}