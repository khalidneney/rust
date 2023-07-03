use core::num;
use std::{collections::HashSet, sync::WaitTimeoutResult, vec};

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut words_hash: Vec<HashSet<char>> = vec![];
        for i in words{
            
        }
        return vec![];
    }
}

fn main(){
    let mut str: String = String::from("dfh");
}
pub fn reformat_number(number: String) -> String {
    let mut number: String = number.chars().filter(|a| *a != '-' && *a != ' ').collect();
    let mut result: String = String::new();
    while number.len() > 4 {
        result.push_str(&number[0..3]);
        result.push('-');
        number = number[3..].to_string();
    }
    if number.len() == 4{
        result.push_str(&number[0..2]);
        result.push('-');
        result.push_str(&number[2..]);
    }else{
        // 3 2
        for i in (0..number.len()).step_by(3){
            result.push_str(&number[..i]);
        }
        result.push_str(&number[..]);
    }
    return result;
}