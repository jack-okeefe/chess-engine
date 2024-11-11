use std::cmp;
use std::collections::HashSet;

use crate::board::{FILES, RANKS};

pub fn algebraic_to_index(algebraic: String) -> Result<usize, &'static str> {
    if algebraic.len() != 2 {
        return Err("Input must be exactly 2 characters long.");
    }

    let file: char = algebraic.chars().nth(0).unwrap();
    let rank: char = algebraic.chars().nth(1).unwrap();

    let file_index = if FILES.contains(&file) {
        file as u8 - b'a'
    } else {
        return Err("Invalid file. Must be between 'a' and 'h'.");
    };

    let rank_index = if RANKS.contains(&rank) {
        rank as u8 - b'1'
    } else {
        return Err("Invalid rank. Must be between '1' and '8'.");
    };

    // Calculate the final index: (rank_index * 8 + file_index)
    let index = ((rank_index * 8) + file_index) as usize;

    Ok(index)
}

pub fn index_to_algebraic(index: &usize) -> Result<String, &'static str> {
    if *index > 63 {
        return Err("Invalid index. Must be less than 64");
    } else {
        let file_index = *index % 8;
        let file = FILES[file_index].to_string();
        let rank = (*index / 8 + 1).to_string();
        let algebraic = format!("{file}{rank}");
        return Ok(algebraic);
    }
}

pub fn bit_scan(bitboard: &u64) -> HashSet<usize> {
    let mut mask = bitboard.clone();
    let mut indicies = HashSet::new();
    let mut index: usize = 0;
    while mask > 0 {
        if mask.trailing_zeros() == 0 {
            indicies.insert(index);
        }
        let shift_by = cmp::max(1, mask.trailing_zeros());
        mask = mask >> shift_by;
        index += shift_by as usize;
    }
    indicies
}

pub fn index_to_bitboard(index: usize) -> u64 {
    1 << index
}
