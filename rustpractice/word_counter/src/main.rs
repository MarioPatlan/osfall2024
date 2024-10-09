fn most_frequent_word(text: &str) -> (String, usize) {

    let word_vec: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = "";
    let mut max_count = 0;

    for word in word_vec.iter() {
        let mut word_count = 0;
        for w in word_vec.iter() {
            if word == w {
                word_count += 1;
            }
        }
        if word_count > max_count {
            max_word = word;
            max_count = word_count;
        }
    }

   return (max_word.to_string(), max_count);

}

fn main() {

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);

}