use std::io::{self, Write};

fn main() {
    let as_nums = get_input("Enter a message to convert: ").chars().map(|x| x as i32).collect::<Vec<i32>>();

    for num in as_nums {
        if num == 32 {
            print!(" ");
            continue;
        }

        print!("{}", num);
    }
}

fn get_input(msg: &str) -> String {
    print!("{}", msg);

    io::stdout().flush().expect("Error flushing output");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading input");

    input.trim().to_string()
}