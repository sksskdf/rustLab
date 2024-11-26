

fn it(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        print!("{} \n", i);
        print!("{} \n", item as char);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let s = String::from("hello ");
    it(&s);

    print!("{}", "hello");
    print!("{}", 0 as char);
    print!("{}", 83 as char);
}
