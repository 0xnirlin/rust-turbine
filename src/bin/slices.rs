// instead of passing in the &String we can pass &str which is slice and is an optimization overtop and works just fine. 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);

    s.clear();
}