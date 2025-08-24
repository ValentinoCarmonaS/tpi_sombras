mod data;
mod flatlander;
mod read_input;
mod run;
mod shadow_error;

use run::run;

fn main() {
    match run() {
        Ok(ans) => println!("{}", ans),
        Err(e) => e.show_error(),
    }
}
