fn most_frequent_word(text: &str) -> (String, usize) {
    let mut max_word = String::new();
    let mut max_count: usize = 0;

    let words: Vec<&str> = text.split_whitespace().collect();
    let n: usize = words.len();
    let mut visited = vec![false; n];
    for i in 0..words.len() {
        let mut count: usize = 1;
        if visited[i] == true {
            //if word has been counted already then skip
            continue;
        }
        for j in (i + 1)..words.len() {
            if words[i] == words[j] {
                count += 1;
                visited[j] = true;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = words[i].to_string();
        }
    }
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
