use std::collections::HashMap;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut wrd1:String=word1.chars().rev().collect();
        let mut wrd2:String=word2.chars().rev().collect();
        let mut toRet=String::new();
        let mut switch:i32=0;
        while wrd1.len()>0 && wrd2.len()>0{
            match switch{
                0=>{if let Some(mut hold)=wrd1.pop(){
                toRet+=&hold.to_string() ;
                switch=1;}
                else{
                    panic!("Oh No");
                }
                },
                1=>{if let Some(mut hold)=wrd2.pop(){
                toRet+=&hold.to_string();
                    switch=0;}
                    else{
                        panic!("Oh Boy!!");
                    }
                
                }
                _=>{}
            }
        }   
        
        while wrd1.len()>0{
            if let Some(mut hold2)=wrd1.pop(){
            toRet+=&hold2.to_string();}
            else{
                panic!("oh no");
            }
        }
        
         while wrd2.len()>0{
            if let Some(mut hold2)=wrd2.pop(){
            toRet+=&hold2.to_string();}
            else{
                panic!("Very Sad");
            }
        }

        return toRet;
        }
}