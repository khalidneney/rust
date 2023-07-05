

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut larges_prefix = String::new();
    let mut first_string = &strs[0];
    for i in 0..first_string.len(){
        let mut flag = 0; 
        for j in 1..strs.len(){
            if first_string.chars().nth(i) == strs[j].chars().nth(i){
                larges_prefix += &first_string.chars().nth(i).unwrap().to_string();
                print!("{}\n", larges_prefix)
            }else{
                flag = 1;
            }
        }
        if flag == 1{
            break;
        }
    }
    if larges_prefix.len() > 0{
        return larges_prefix;
    }
    return "".to_string();
}


fn main(){
    print!("{}\n", longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]));

}