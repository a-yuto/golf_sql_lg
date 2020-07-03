use std::collections::HashSet;

fn newline(oneline: &str) -> String{
    let reserved_word : HashSet<String>= vec!["SELECT".to_string(),"FROM".to_string(),"WHERE".to_string()].into_iter().collect();
    let mut now_word = "".to_string();
    let mut ans = "".to_string();
    for ch in oneline.chars() {
        if ch == ' '{
            if reserved_word.contains(&now_word){
                ans = format!("{}\n{}",ans,now_word);    
            }else{
                ans = format!("{} {}",ans,now_word);
            }
            now_word = "".to_string();
        }else{
            now_word.push(ch);
        }
    }
    ans = format!("{} {}",ans,now_word);
    ans[1..].to_string()
}

fn relation(sql: &str) -> String {
    let mut start = "".to_string();
    let mut end   = "".to_string();
    let mut now_word = "".to_string();
    let mut with_flag = false;
    let mut from_flag = false;
    for ch in sql.chars() {
        if ch == ' '{
            if with_flag {
                start = now_word;
                with_flag  = false;
            }
            if now_word == "WITH"{
                now_word = "".to_string();
                with_flag = true;
            } 

            if from_flag {
                end        = now_word;
                from_flag  = false;
            }
            if now_word == "FROM"{
                now_word = "".to_string();
                from_flag = true;
            } 
        }else{
            now_word.push(ch);
        }
    }
    fn hoge(sql: &str, now_word: &str,index: usize,with_flag: bool,from_flag: bool,start: &str,end: &str) -> String {
        if ch == " " {
            if with_flag {
                hoge(sql,"",index + 1,false,false,now_word,end);
            }
            if now_word == "WITH" {
                hoge(sql,"",index + 1,true,false,start,end);
            }
            if from_flag {
                hoge(sql,"",index + 1,false,false,now_word,now_word);
            }
            if now_word == "FROM" {
                hoge(sql,"",index + 1,false,true,start,end);
            }
        }
        else {
            
        }
    }
    format!("{} → {}",start,end)
}

fn main() {
    println!("{}",newline("SELECT * FROM hoge WHERE fuga"));
}

#[test]
fn newline_test1(){
    let ans = 
"SELECT *
FROM hoge".to_string();
    let test = "SELECT * FROM hoge";
    assert_eq!(ans,newline(test));
}

#[test]
fn newline_test2(){
    let ans =
"SELECT *
FROM hoge
WHERE fuga".to_string();
    let test = "SELECT * FROM hoge WHERE fuga";
    assert_eq!(ans,newline(test));
}

#[test]
fn relation_test() {
    let ans  = "hoge → fuga".to_string();
    let test = 
    "WITH hoge AS (
        SELECT *
        FROM fuga
    )";
    assert_eq!(ans,relation(&test));
}
