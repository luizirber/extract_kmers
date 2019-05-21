use std::collections::HashMap;

pub const codon_table: HashMap<&str, &str> =
    
    [("TTT", "F"), ("TTC", "F"),
    ("TTA", "L"), ("TTG", "L"),

    ("TCT", "S"), ("TCC", "S"), ("TCA", "S"), ("TCG", "S"), ("TCN", "S"),

    ("TAT", "Y"), ("TAC", "Y"),
    ("TAA", "*"), ("TAG", "*"),

    ("TGT", "C"), ("TGC", "C"),
    ("TGA", "*"),
    ("TGG", "W"),

    ("CTT", "L"), ("CTC", "L"), ("CTA", "L"), ("CTG", "L"), ("CTN", "L"),

    ("CCT", "P"), ("CCC", "P"), ("CCA", "P"), ("CCG", "P"), ("CCN", "P"),

    ("CAT", "H"), ("CAC", "H"),
    ("CAA", "Q"), ("CAG", "Q"),

    ("CGT", "R"), ("CGC", "R"), ("CGA", "R"), ("CGG", "R"), ("CGN", "R"),

    ("ATT", "I"), ("ATC", "I"), ("ATA", "I"),
    ("ATG", "M"),

    ("ACT", "T"), ("ACC", "T"), ("ACA", "T"), ("ACG", "T"), ("ACN", "T"),

    ("AAT", "N"), ("AAC", "N"),
    ("AAA", "K"), ("AAG", "K"),

    ("AGT", "S"), ("AGC", "S"),
    ("AGA", "R"), ("AGG", "R"),

    ("GTT", "V"), ("GTC", "V"), ("GTA", "V"), ("GTG", "V"), ("GCN", "V"),

    ("GCT", "A"), ("GCC", "A"), ("GCA", "A"), ("GCG", "A"), ("GCN", "A"),

    ("GAT", "D"), ("GAC", "D"),
    ("GAA", "E"), ("GAG", "E"),

    ("GGT", "G"), ("GGC", "G"), ("GGA", "G"), ("GGG", "G"), ("GGN", "G")]
    .iter().cloned().collect();
