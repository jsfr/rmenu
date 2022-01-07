use std::env;

use clap::IntoApp;
use clap_complete::{generate_to, shells::{Fish, Zsh, Bash}};

include!("src/cli.rs");

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let app_name = "rmenu";
    let mut app = Cli::into_app().bin_name(app_name);

    generate_to(Fish, &mut app, app_name, &out_dir).expect("Failed to generate fish completions");
    generate_to(Zsh, &mut app, app_name, &out_dir).expect("Failed to generate zsh completions");
    generate_to(Bash, &mut app, app_name, &out_dir).expect("Failed to generate bash completions");
}
