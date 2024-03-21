use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut H1:HashMap<String,Vec<String>>=HashMap::new();
        let mut anagrams:Vec<Vec<String>>=vec![];
        for i in strs{
            let mut chars: Vec<char> = i.chars().collect();
            chars.sort();


            H1.entry(chars.iter().collect()).or_insert(vec![]).push(i); 
            
        }
        let mut anagrams:Vec<Vec<String>>=H1.values().cloned().collect();
      
        return anagrams;
    }

    
}