use std::time::Duration;

use entoli::{
    control::concurrent::{delay_for, rec},
    prelude::{get_line, put_str_ln, Io},
};

fn main() {
    let ask_and_sleep = put_str_ln("How long should I sleep for? (in seconds)")
        .then(get_line)
        .and_then(|s| {
            put_str_ln("Sleeping...").then(delay_for(Duration::from_secs(s.parse().unwrap())))
        })
        .then(put_str_ln("Done!"));

    let main = rec(ask_and_sleep);

    main.run();
}
