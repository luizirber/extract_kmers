extern crate lazy_static;
extern crate needletail;
use exitfailure::ExitFailure;
extern crate clap;
use clap::{App, load_yaml, value_t};

mod extract;

fn main() -> Result<(), ExitFailure> {

    let yml = load_yaml!("must.yml");
    let m = App::from_yaml(yml).get_matches();

    match m.subcommand_name() {
        Some("extract") => {
            let cmd = m.subcommand_matches("compute").unwrap();
            let sequence_files = cmd
                .values_of("sequence_files")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap();

            // Convert ksize string argument to integer
            let ksize: u8 = value_t!(cmd, "ksize", u8).unwrap_or_else(|e| e.exit());
            extract::extract_kmers(sequence_files, ksize);
        }
        _ => {
            println!("{:?}", m);
        }
    }
    Ok(())
}
