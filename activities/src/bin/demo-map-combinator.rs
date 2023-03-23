fn maybe_num() -> Option<i32> {}

fn maybe_word() -> Option<String> {}

fn main() {
    let plus_one = match maybe_num() {
        Some(num) => Some(num + 1),
        None => None,
    };

    let plus_one = maybe_num().map(|i| i + 1);

    let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);
}
