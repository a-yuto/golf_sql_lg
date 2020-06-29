use std::collections::HashSet;

fn newline(oneline: &str) -> String{
    let reserved_word : HashSet<String>= vec!["SELECT".to_string(),"FROM".to_string()].into_iter().collect();
    let mut now_word = "".to_string();
    let mut ans = "".to_string();
    for ch in oneline.chars() {
        if ch == ' '{
            if reserved_word.contains(&now_word){
                ans = format!("{} \n{}",ans,now_word);    
            }else{
                ans = format!("{} {}",ans,now_word);
            }
            now_word = "".to_string();
        }else{
            now_word.push(ch);
        }
    }
    ans = format!("{} {}",ans,now_word);
    ans[2..].to_string()
}

fn main() {
    println!("{}",newline("SELECT * FROM hoge"));
}

#[test]
fn newline_test(){
    let ans = 
"SELECT * 
FROM hoge".to_string();
    let test = "SELECT * FROM hoge";
    assert_eq!(ans,newline(test));
}
