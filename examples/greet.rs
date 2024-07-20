use entoli::prelude::Io;
use entoli::prelude::{get_line, put_str_ln};

fn main() {
    let main = put_str_ln("What is your name?")
        .then(get_line)
        .and_then(|s| put_str_ln(format!("Hello, {}!", s)));

    main.run();
}
