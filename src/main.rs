// Copyright (c) Jason Ekstrand 2022.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice,
//     this list of conditions and the following disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright
//     notice, this list of conditions and the following disclaimer in the
//     documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its
//     contributors may be used to endorse or promote products derived from
//     this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Word {
    index: u16,
    letters: u32,
}

fn main() {
    // Open the file name given as a command line argument
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap();

    // Read the file and filter to only 5-letter words
    let lines = io::BufReader::new(file).lines();
    let words: Vec<String> =
        lines.map(|w| w.unwrap()).filter(|w| w.len() == 5).collect();

    // The total number of words should fit in 16 letters
    let num_words: u16 = words.len().try_into().unwrap();

    // We only care about which letters are in a word, not the order of the
    // letters.  This means, we can represent each word as a u32 with one bit
    // set for each letter in the English alphabet.  This also makes it
    // trivially easy to check if two words contain any of the same letters by
    // using AND and to combine two words by using OR.
    let mut singles: Vec<Word> = Vec::new();
    for i in 0..num_words {
        let mut letters = 0u32;
        for c in words[i as usize].to_uppercase().as_bytes() {
            assert!(b'A' <= *c && *c <= b'Z');
            letters |= 1u32 << (*c - b'A');
        }
        if letters.count_ones() == 5 {
            singles.push(Word {
                index: i,
                letters: letters,
            });
        }
    }

    // Get rid of any annograms.  This isn't necessary but cuts down on
    // runtime and lets us verify results against Matt's algorithm
    singles.sort_by_key(|x| x.letters.reverse_bits());
    singles.dedup_by_key(|x| x.letters);

    let num_singles: u16 = singles.len().try_into().unwrap();

    let mut num_sets_2: u32 = 0;
    let mut num_sets_3: u32 = 0;
    let mut num_sets_4: u32 = 0;
    let mut num_sets_5: u32 = 0;

    for a in 0..num_singles {
        let sa = &singles[a as usize];
        let letters1 = sa.letters;
        for b in (a+1)..num_singles {
            let sb = &singles[b as usize];

            if (letters1 & sb.letters) != 0 {
                continue;
            }
            num_sets_2 += 1;
            let letters2 = letters1 | sb.letters;

            for c in (b+1)..num_singles {
                let sc = &singles[c as usize];

                if (letters2 & sc.letters) != 0 {
                    continue;
                }
                num_sets_3 += 1;
                let letters3 = letters2 | sc.letters;

                for d in (c+1)..num_singles {
                    let sd = &singles[d as usize];

                    if (letters3 & sd.letters) != 0 {
                        continue;
                    }
                    num_sets_4 += 1;
                    let letters4 = letters3 | sd.letters;

                    for e in (d+1)..num_singles {
                        let se = &singles[e as usize];

                        if (letters4 & se.letters) != 0 {
                            continue;
                        }
                        num_sets_5 += 1;

                        println!("{}, {}, {}, {}, {}",
                                 words[sa.index as usize],
                                 words[sb.index as usize],
                                 words[sc.index as usize],
                                 words[sd.index as usize],
                                 words[se.index as usize]);
                    }
                }
            }
        }
    }
    println!("Found {} words with unique letters", num_singles);
    println!("Found {} pairs of words with unique letters", num_sets_2);
    println!("Found {} sets of three words with unique letters", num_sets_3);
    println!("Found {} sets of four words with unique letters", num_sets_4);
    println!("Found {} sets of five words with unique letters", num_sets_5);
}
