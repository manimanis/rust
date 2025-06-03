fn print_str(s: &str) {
    let ns = format!("{}!", s);
    println!("{}", ns);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "Hello world";
    print_str(s);

    let sal = String::from("Hello");
    print_string(sal);

    let sent = "the quick brown fox jumps over the lazy dog";
    println!("{}", &sent[0..3]);

    let desc = format!("Title: Quick story\n{}", sent);
    println!("{}", desc);

    for c in sent.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel: {}", c),
            _ => continue,
        }
    }

    let words: Vec<&str> = sent.split_whitespace().rev().collect();
    println!("{:?}", words);
}
