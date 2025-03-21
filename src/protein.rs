use seq_io::fasta::{Record, RefRecord};
use rand::seq::index::sample;
use boomphf::*;
use std::fmt;

type BitwiseAminoAcid = u8;
type FiveMer = u32;
type SevenMer = u32;

const AMINO_ACID_LIST: [char; 21] = [
    'C', 'S', 'T', 'A', 'G',
    'P', 'D', 'E', 'Q', 'N',
    'H', 'R', 'K', 'M', 'I', 
    'L', 'V', 'W', 'Y', 'F', '*'];

fn bitwise_power(mut a: u32, mut n: u32) -> u32 {
    let mut ans: u32 = 1;
    while n > 0 {
        let odd: u32 = n & 1u32;        // is 1 if n is odd, else is zero
        if odd > 0u32 {                 // if odd
            ans *= a;                   // can multiply
        }
        if n > 1 {
            a = a * a;                  // raise a by 2
        }
        n = n >> 1;                     // floor divide n by 2
    }
    ans
}
fn create_five_mer(amino_acids: [BitwiseAminoAcid; 5]) -> FiveMer {
    let powers_of_twenty_one = [
        1, 21, 21*21, bitwise_power(21, 3), bitwise_power(21, 4)];
    let mut five_mer: u32= 0u32;
    for i in 0..5 {
        five_mer += Into::<u32>::into(amino_acids[i]) * powers_of_twenty_one[4-i];
    }
    five_mer
}
fn five_mer_back_to_amino_acid(five_mer: FiveMer) -> String {
    let powers_of_twenty_one = [
        1, 21, 21*21, bitwise_power(21, 3), bitwise_power(21, 4)];
    let mut amino_acids = String::new();
    let mut five_mer_clone = five_mer.clone();
    for i in 0..5 {
        amino_acids.push(AMINO_ACID_LIST[(five_mer_clone / powers_of_twenty_one[4-i]) as usize]);
        five_mer_clone %= powers_of_twenty_one[4-i];
    }
    amino_acids
}
fn create_seven_mer(amino_acids: [BitwiseAminoAcid; 7]) -> SevenMer {
    let powers_of_twenty_one = [
        1, 21, 21*21, bitwise_power(21, 3), bitwise_power(21, 4),
        bitwise_power(21, 5), bitwise_power(21, 6)];
    let mut seven_mer: u32= 0u32;
    for i in 0..7 {
        seven_mer += Into::<u32>::into(amino_acids[i]) * powers_of_twenty_one[6-i];
    }
    seven_mer
}
fn amino_acid_to_bits(amino_acid: &char) -> BitwiseAminoAcid {
    let index_usize = AMINO_ACID_LIST.iter().position(|x| x==amino_acid)
        .unwrap_or(20usize);
    let index_u8: u8 = index_usize.try_into().unwrap();
    index_u8
}


#[derive(Default)]
#[derive(Clone)]
pub struct Protein {
    id: String,
    rand_five_mers: Vec<FiveMer>,
    rand_seven_mers: Vec<SevenMer>,
    hash_five_mers: Vec<bool>,
    hash_seven_mers: Vec<bool>,
}
impl fmt::Debug for Protein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let five_mers: Vec<String> =  self.rand_five_mers.iter()
                .map(|x| five_mer_back_to_amino_acid(*x)).collect();
        f.debug_struct("Protein")
            .field("id", &self.id)
            .field("5-mers", &five_mers)
            .finish()
    }
}
impl Protein {
    pub fn new_protein(record: &RefRecord) -> Protein {
        let mut rng = rand::rng();
        let id = record.id().unwrap().to_string();
        let seq = record.seq();
        let seq_len = seq.len();
        let rand_five_mer_indices = sample(
            &mut rng, seq_len-4, (seq_len-4)/10).into_vec();
        let rand_seven_mer_indices = sample(
            &mut rng, seq_len-6, (seq_len-6)/10).into_vec();
        let rand_five_mers: Vec<FiveMer> = rand_five_mer_indices.iter().map(
            |start| {
                let end = start + 5;
                let mut amino_acids = [0u8; 5];
                for i in *start..end {
                    amino_acids[i-start] = amino_acid_to_bits(&(seq[i] as char));
                }
                create_five_mer(amino_acids)
            }).collect();
        let rand_seven_mers: Vec<SevenMer> = rand_seven_mer_indices.iter().map(
            |start| {
                let end = start + 7;
                let mut amino_acids = [0u8; 7];
                for i in *start..end {
                    amino_acids[i-start] = amino_acid_to_bits(&(seq[i] as char));
                }
                create_seven_mer(amino_acids)
            }).collect();
        let hash_five_mers: Vec<bool> = vec![];
        let hash_seven_mers: Vec<bool> = vec![];
        Protein {
            id,
            rand_five_mers,
            rand_seven_mers,
            hash_five_mers,
            hash_seven_mers,
        }
    }
    pub fn get_five_mers(&self) -> Vec<FiveMer> {
        return self.rand_five_mers.clone();
    }
    pub fn get_seven_mers(&self) -> Vec<SevenMer> {
        return self.rand_seven_mers.clone();
    }
    pub fn get_five_hash(&self) -> Vec<bool> {
        return self.hash_five_mers.clone();
    }
    pub fn get_seven_hash(&self) -> Vec<bool> {
        return self.hash_seven_mers.clone();
    }
    pub fn modify_hash_five_mer(&mut self, len: &usize, phf: &Mphf<u32>) {
        let mut hash_map = vec![false; *len];
        for five_mer in &self.rand_five_mers {
            hash_map[phf.hash(five_mer) as usize] = true;
        }
        self.hash_five_mers = hash_map;
    }
    pub fn modify_hash_seven_mer(&mut self, len: &usize, phf: &Mphf<u32>) {
        let mut hash_map = vec![false; *len];
        for seven_mer in &self.rand_seven_mers {
            hash_map[phf.hash(seven_mer) as usize] = true;
        }
        self.hash_seven_mers = hash_map;
    }
}
