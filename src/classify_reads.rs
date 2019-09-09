use std::str;
use std::collections::HashSet;
extern crate lazy_static;
extern crate needletail;
use needletail::{fastx};
extern crate clap;
use clap::{Arg, App, value_t};

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_kmer_file(kmer_file: str) {
    let mut kmers = HashSet::new();

    // Read in coding k-mers
    let f = File::open(kmer_file)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let kmer = line.unwrap();
        kmers.insert(kmer.to_owned());
    }
    return kmers;
}

fn jaccardize(set1: HashSet, set2: HashSet, verbose: bool) -> f64 {
    let denominator = min(set1.len(), set2.len());
    if denominator > 0 {
        numerator = set1.intersection(set2).len();
        if verbose {
            println("Number of overlapping k-mers: ", numerator, "out of", denominator)
        }
        return numerator/denominator;
    } else {
        return 0.0;
    }
}

fn main() {
    let matches = App::new("Classify reads as coding or non-coding")
        .version("1.0")
        .author("Olga Botvinnik <olga.botvinnik@czbiohub.org>")
        .about("Classifies reads as coding or noncoding")
        .arg(Arg::with_name("sequence_files")
            .short("f")
            .long("sequence_files")
            .value_name("SEQUENCE_FILES")
            .help("Input sequence files")
            .required(true)
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name("coding_kmers")
            .long("coding_kmers")
            .value_name("CODING_KMERS")
            .help("Plain text file of k-mers extracted from coding sequences (e.g. Homo_sapiens.GRCh38.cds.all.fa.gz )")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("noncoding_kmers")
            .long("noncoding_kmers")
            .value_name("noncoding_KMERS")
            .help("Plain text file of k-mers extracted from noncoding sequences (e.g. Homo_sapiens.GRCh38.ncrna.fa.gz )")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("ksize")
            .short("k")
            .long("ksize")
            .value_name("KSIZE")
            .help("k-mer size")
            // Clap currently only handles strings. Have to convert to int later
            .default_value("21")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    let sequence_files = matches.values_of("sequence_files").unwrap();
    let coding_kmer_file = matches.value_of("coding_kmers").unwrap();
    let noncoding_kmer_file = matches.value_of("noncoding_kmers").unwrap();

    // Convert ksize string argument to integer
    let ksize = value_t!(matches, "ksize", u8).unwrap_or_else(|e| e.exit());
    let mut coding_kmers = read_kmer_file(coding_kmer_file);
    let mut noncoding_kmers = read_kmer_file(noncoding_kmer_file);

    for file in files {
        eprintln!("Counting k-mers in file: {}", file);

        let mut n_bases = 0;
        fastx::fastx_cli(&file[..], |_| {}, |seq| {
            let mut this_read_kmers = HashSet::new();
            // seq.id is the name of the record
            // seq.seq is the base sequence
            // seq.qual is an optional quality score

            // keep track of the total number of bases
            n_bases += seq.seq.len();

            // keep track of the number of AAAA (or TTTT via canonicalization) in the
            // file (normalize makes sure every base is capitalized for comparison)
            for (_, kmer, _) in seq.normalize(false).kmers(ksize, false) {
                let kmer = str::from_utf8(&kmer).unwrap();
                this_read_kmers.insert(kmer.to_owned());
//            println!("{}", kmer);
            }
            let this_read_kmers_in_coding = coding_kmers.intersection(&this_read_kmers);
            let this_read_kmers_in_noncoding = noncoding_kmers.intersection(&this_read_kmers);

        }).expect(&format!("Could not read {}", file));


        eprintln!("There are {} bases in your file.", n_bases);
    }
    eprintln!("There are {} k-mers in your file.", all_kmers.len());

    for kmer in all_kmers {
        println!("{}", kmer);
    }

}
