fn main() {
    let mut tup: (i32, f64, String);
    tup = (3, std::f64::consts::PI, String::from("PI"));
    println!("{:?}", tup);
    tup.2.push_str(" is a constant");
    println!("{:?}", tup);

    let mut s: String = String::from("hello\u{2301}");
    s.push_str(", world");
    println!("{}", s);

    let last_word = &s[11..];
    println!("last_word is {}", last_word);

    let grades: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let passing_grades: &[u32] = &grades[4..];
    println!("passing_grades is {:?}", passing_grades);

    let message = String::from("FCSB \u{2726} e STEAUA \u{22C6}");
    println!("message is {}", message);
    let bytes = message.as_bytes();
    let mut counter: u32 = 0;
    for (index, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            counter += 1;
        }
    }
    /*for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            counter += 1;
        }
    }*/
    for (index, item) in message.chars().enumerate() {
        println!("{} {}", index, item);
    }
    println!("counter is {}", counter);
    for (index, item) in message.chars().enumerate() {
        println!("{}", index);
    }

    let message = "     TAta mare          ";
    println!("message is {}", message);
    println!("message is {}", trim_spaces(&message));

    println!("START TESTS: ");
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");

    let test = "word1 word2 word3 word4";
    println!("{}", second_word(&test));

    let test = "word2";
    println!("{}", second_word(&test));

    let test = "";
    println!("{}", second_word(&test));
}

fn second_word(text: &str) -> &str {
    let mut start_index = 0;
    let mut end_index = 0;
    let mut counter = 0;
    for (i, c) in text.chars().enumerate() {
        if c == ' '{
            if counter == 0 {
                start_index = i;
            } else if counter == 1 {
                end_index = i;
            }
            counter += 1;
        }
    }
    if start_index < end_index {&text[start_index+1..end_index]} else {""}

}

fn trim_spaces(text: &str) -> &str {
    if text.len() == 0{
        return "";
    }
    let mut start = 0;
    let mut finish = text.len() - 1;
    while start < text.len() && check_char(&text.as_bytes()[start])  {
        start += 1;
    }
    if start == text.len(){
        return "";
    }
    else {
        while 0 <= finish && check_char(&text.as_bytes()[finish]) {
            finish -= 1;
        }
        if start <= finish { &text[start..finish+1] } else { "" }
    }

}

fn check_char(ch: &u8) -> bool {
    *ch == b' ' || *ch == b'\t' || *ch == b'\r' || *ch == b'\n'
}
