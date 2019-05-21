extern crate needletail;
use needletail::{fastx};
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("files")
            .short("f")
            .long("files")
            .value_name("FILES")
            .help("Input sequence file")
            .required(true)
            .multiple(true)
            .takes_value(true))
        .get_matches();

    let files = matches.values_of("files").unwrap();

    for file in files {
        println!("Counting k-mers in file: {}", file);

        let mut n_bases = 0;
        let mut n_valid_kmers = 0;
        fastx::fastx_cli(&file[..], |_| {}, |seq| {
        // seq.id is the name of the record
        // seq.seq is the base sequence
        // seq.qual is an optional quality score

        // keep track of the total number of bases
        n_bases += seq.seq.len();

        // keep track of the number of AAAA (or TTTT via canonicalization) in the
        // file (normalize makes sure ever base is capitalized for comparison)
        for (_, kmer, _) in seq.normalize(false).kmers(4, true) {
        if kmer == b"AAAA" {
        n_valid_kmers += 1;
        }
        }
        }).expect(&format!("Could not read {}", file));

        println!("There are {} bases in your file.", n_bases);
        println!("There are {} AAAAs in your file.", n_valid_kmers);
    }

}


