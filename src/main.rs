extern crate core;

use std::process;

use hetzner_dyndns_updater::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = hetzner_dyndns_updater::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
