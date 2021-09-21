use clap::App;

mod caesar;
mod cli;
mod io;

fn main() {
    let yaml = clap::load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .get_matches();

    cli::run(matches);
}
