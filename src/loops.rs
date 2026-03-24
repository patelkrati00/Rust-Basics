pub fn loops(){
    // for i in 0..9{
    //     print!("{} ",i);
    // }
    let sentence =  String::from("for loop in rust");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);

}

fn get_first_word( sentence: String) -> String{
    let mut ans = String::from("");
    for ch in sentence.chars(){
        ans.push_str(ch.to_string().as_str()); 
        if ch == ' '{
            break ;
        }
    }
    return ans;
}
