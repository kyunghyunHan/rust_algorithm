
pub fn main(){
    // let ferris= "Ferris".to_string();
    // let curious= "Curious".to_string();
    // println!("{}: {}",ferris,three_vowels(&ferris));
    // println!("{}: {}", curious, three_vowels(&curious));

    let sentence_string =
    "Once upon a time, there was a friendly curious crab named Ferris".to_string();

    for word in sentence_string.split(' '){
        if three_vowels(word){
            println!("{} has three consecutive vowels!", word);
        }
    }
}

fn three_vowels(word:&str)->bool{

    let mut vowel_count= 0;
    for c in word.chars() {
        match c {
            'a'|'e'|'i'|'o'|'u'=>{
                vowel_count += 1;
                if vowel_count >=3 {
                    return true
                }
            }
            _=>vowel_count =0
        }
    }
false
}