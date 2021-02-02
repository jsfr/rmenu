use std::env;

use clap::IntoApp;
use clap_generate::{generate_to, generators::{Fish, Zsh, Bash}};

include!("src/cli.rs");

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut app = Cli::into_app().bin_name("rmenu_history");

    generate_to::<Fish, _, _>(&mut app, "rmenu_history", &out_dir);
    generate_to::<Bash, _, _>(&mut app, "rmenu_history", &out_dir);
    generate_to::<Zsh, _, _>(&mut app, "rmenu_history", &out_dir);
}
