use strsim::levenshtein;


fn correct_word(myword: & str, words: &[&str]) -> &str {
    let newarest_word = "";
    let mut distance = usize::MAX;

    for &word in words {
        // if word == myword {
        //     println!("Found String");
        // }
        // find distance
        if distance > levenshtein(word, myword) {
            newarest_word = word;
            println!("Found nearest word so far = {}", word);
        }
    }

    newarest_word
}


fn main() {
    let words = ["apple", "bananna", "pomogranite"];
    let mut myword = "apple";

    correct_word(& myword, &words);

    println!("Hello, world!");
}
