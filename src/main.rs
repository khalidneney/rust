use core::num;


fn main(){
    let sdtr: String = String::from("hello world!");
    let strd = &sdtr[2..3];
    print!("{}", reformat_number(String::from("123-456 7845")));
}
pub fn reformat_number(number: String) -> String {
    let mut number: String = number.chars().filter(|a| *a != '-' && *a != ' ').collect();
    let mut result: String = String::new();
    while number.len() > 4 {
        result.push_str(&number[0..3]);
        number = number[3..].to_string();
    }
    if result.len() == 4{
        result.push_str(&number[0..2]);
        result.push('-');
        result.push_str(&number[2..]);
    }else{

    }
    return result;
}