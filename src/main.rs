mod shadow_error;
mod read_input;
mod run;
mod data;
mod flatlander;

use run::run;

fn main() {
    match run() {
        Ok(ans) => println!("{}", ans),
        Err(e) => e.show_error(),
    }
}
