/*
Convert strings to pig latin.
The first consonant of each word is moved
to the end of the word and “ay” is added,
so “first” becomes “irst-fay.”

Words that start with a vowel have “hay”
added to the end instead (“apple” becomes “apple-hay”).

Keep in mind the details about UTF-8 encoding!

// vowels - a, e, i, o, u
// consonants - the rest
*/

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let strings = "Lorem ipsum dolor sit amet consectetuer adipiscing elit Aenean commodo ligula eget dolor Aenean massa.";
    let words = strings.split(" ");

    for word in words {
        if (word.is_empty()) {
            continue;
        }
        //only support ascii for simplicity.
        let first_char = word.chars().nth(0).expect("No first character!");
        let lowercase_char = first_char.to_ascii_lowercase();

        if VOWELS.contains(&lowercase_char) {
            println!("{word} => {word}-hay ")
        } else {
            let mut word_without_first = word.chars();
            word_without_first.next();
            let word_without_first = word_without_first.as_str();

            let first_char = word.chars().nth(0).expect("No first character!");
            println!("{word} => {word_without_first}-{first_char}ay ");   
        }
    }
}
