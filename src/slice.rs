// 这个方法我们不得不时刻担心的索引与 s 中的数据不再同步，这很啰嗦且易出错
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_two(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn main() {
    let mut s = String::from("hello world");

    let word = first_word_two(&s);

    s.clear();

    // println!("the first word is: {}", word);
}
