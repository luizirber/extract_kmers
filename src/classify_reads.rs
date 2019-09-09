use std::str;
use std::cmp::min;
use std::collections::HashSet;
extern crate lazy_static;
extern crate needletail;
use needletail::{fastx};

use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_kmer_file(kmer_file: &str) -> HashSet<&'static str> {
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

fn jaccardize(set1: &HashSet<&str>, set2: &HashSet<&str>, verbose: usize) -> f64 {
    let denominator = min(set1.len(), set2.len());
    if denominator > 0 {
        let numerator = set1.intersection(&set2).len();
        if verbose {
            println!("Number of overlapping k-mers: {numerator}/{denominator}",
                     numerator=numerator, denominator=denominator)
        }
        return numerator/denominator;
    } else {
        return 0.0;
    }
}

pub fn classify(sequence_files: Vec<&str>, coding_kmer_file: str,
                non_coding_kmer_file: str, ksize: u8, verbosity: usize) {
    let coding_kmers = read_kmer_file(&coding_kmer_file);
    let non_coding_kmers = read_kmer_file(&non_coding_kmer_file);
    let mut all_kmers = HashSet::new();

    for file in sequence_files {
        eprintln!("Classifying reads in file: {}", file);

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
                all_kmers.insert(kmer.to_owned());
//            println!("{}", kmer);
            }
            let this_read_kmers_in_coding = coding_kmers.intersection(&this_read_kmers);
            let this_read_kmers_in_noncoding = non_coding_kmers.intersection(&this_read_kmers);
            let jaccard_coding = jaccardize(&this_read_kmers, &coding_kmers, verbosity);
            let jaccard_non_coding = jaccardize(&this_read_kmers, &non_coding_kmers, verbosity);
            if verbosity > 0 {
                println!("{seq} jaccard with coding: {jaccard}", seq=seq.id, jaccard=jaccard_coding);
                println!("{seq} jaccard with non coding: {jaccard}", seq=seq.id, jaccard=jaccard_non_coding);
            }

        }).expect(&format!("Could not read {}", file));


        eprintln!("There are {} bases in your file.", n_bases);
    }
    eprintln!("There are {} k-mers in your file.", all_kmers.len());

}
