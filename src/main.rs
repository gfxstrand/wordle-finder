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

fn read_words(filename: &String) -> Vec<String> {
    let file = File::open(&Path::new(filename)).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|w| w.unwrap()).filter(|w| w.len() == 5).collect()
}

struct SingleWord {
    index: u16,
    bits: u32,
}

fn find_5words_dumb(words: &Vec<String>) {
    let num_words: u16 = words.len().try_into().unwrap();

    // Our first clever observation is that we only care about which letters
    // are in a word, not the order of the letters.  This means, we can
    // represent each word as a u32 with one bit set for each letter in the
    // English alphabet.  This also makes it trivially easy to check if two
    // words contain any of the same letters by using AND and to combine two
    // words by using OR.
    let mut singles: Vec<SingleWord> = Vec::new();
    for i in 0..num_words {
        let mut bits = 0u32;
        for c in words[i as usize].to_uppercase().as_bytes() {
            assert!(b'A' <= *c && *c <= b'Z');
            bits |= 1u32 << (*c - b'A');
        }
        if bits.count_ones() == 5 {
            singles.push(SingleWord {
                index: i,
                bits: bits,
            });
        }
    }

    // Get rid of any annograms
    singles.sort_by_key(|x| x.bits.reverse_bits());
    singles.dedup_by_key(|x| x.bits);

    let num_singles: u16 = singles.len().try_into().unwrap();

    let mut num_sets_2: u32 = 0;
    let mut num_sets_3: u32 = 0;
    let mut num_sets_4: u32 = 0;
    let mut num_sets_5: u32 = 0;

    for a in 0..num_singles {
        let sa = &singles[a as usize];
        let bits1 = sa.bits;
        for b in (a+1)..num_singles {
            let sb = &singles[b as usize];

            if (bits1 & sb.bits) != 0 {
                continue;
            }
            num_sets_2 += 1;
            let bits2 = bits1 | sb.bits;

            for c in (b+1)..num_singles {
                let sc = &singles[c as usize];

                if (bits2 & sc.bits) != 0 {
                    continue;
                }
                num_sets_3 += 1;
                let bits3 = bits2 | sc.bits;

                for d in (c+1)..num_singles {
                    let sd = &singles[d as usize];

                    if (bits3 & sd.bits) != 0 {
                        continue;
                    }
                    num_sets_4 += 1;
                    let bits4 = bits3 | sd.bits;

                    for e in (d+1)..num_singles {
                        let se = &singles[e as usize];

                        if (bits4 & se.bits) != 0 {
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

struct WordPair {
    singles: [u16; 2],
    bits: u32,
}

struct WordTripple {
    singles: [u16; 3],
    bits: u32,
}

struct WordQuad {
    singles: [u16; 4],
    bits: u32,
}

fn find_5words_1(words: &Vec<String>) {
    let num_words: u16 = words.len().try_into().unwrap();

    // Our first clever observation is that we only care about which letters
    // are in a word, not the order of the letters.  This means, we can
    // represent each word as a u32 with one bit set for each letter in the
    // English alphabet.  This also makes it trivially easy to check if two
    // words contain any of the same letters by using AND and to combine two
    // words by using OR.
    let mut singles: Vec<SingleWord> = Vec::new();
    for i in 0..num_words {
        let mut bits = 0u32;
        for c in words[i as usize].to_uppercase().as_bytes() {
            assert!(b'A' <= *c && *c <= b'Z');
            bits |= 1u32 << (*c - b'A');
        }
        if bits.count_ones() == 5 {
            singles.push(SingleWord {
                index: i,
                bits: bits,
            });
        }
    }
    let num_singles: u16 = singles.len().try_into().unwrap();

//    // Get rid of any duplicates
//    singles.sort_by_key(|x| x.bits);
//    singles.dedup_by_key(|x| x.bits);

    println!("Found {} words with unique letters", singles.len());

    let mut pairs: Vec<WordPair> = Vec::new();
    for i in 0..num_singles {
        let si = &singles[i as usize];
        for j in (i+1)..num_singles {
            let sj = &singles[j as usize];
            if (si.bits & sj.bits) == 0 {
                pairs.push(WordPair {
                    singles: [i, j],
                    bits: si.bits | sj.bits,
                });
            }
        }
    }
    println!("Found {} pairs of words with unique letters", pairs.len());

    let mut tripples: Vec<WordTripple> = Vec::new();
    for p in pairs {
        let start = p.singles.iter().max().unwrap() + 1;
        for i in start..num_singles {
            let s = &singles[i as usize];
            if (p.bits & s.bits) == 0 {
                tripples.push(WordTripple {
                    singles: [p.singles[0], p.singles[1], i],
                    bits: p.bits | s.bits,
                });
            }
        }
    }
    println!("Found {} sets of three words with unique letters", tripples.len());

    let mut quads: Vec<WordQuad> = Vec::new();
    for t in tripples {
        let start = t.singles.iter().max().unwrap() + 1;
        for i in start..num_singles {
            let s = &singles[i as usize];
            if (t.bits & s.bits) == 0 {
                quads.push(WordQuad {
                    singles: [t.singles[0], t.singles[1], t.singles[2], i],
                    bits: t.bits | s.bits,
                });
            }
        }
    }
    println!("Found {} sets of four words with unique letters", quads.len());

    for q in quads {
        let start = q.singles.iter().max().unwrap() + 1;
        for i in start..num_singles {
            let s = &singles[i as usize];
            if (q.bits & s.bits) == 0 {
                println!("{}, {}, {}, {}, {}",
                         words[singles[q.singles[0] as usize].index as usize],
                         words[singles[q.singles[1] as usize].index as usize],
                         words[singles[q.singles[2] as usize].index as usize],
                         words[singles[q.singles[3] as usize].index as usize],
                         words[s.index as usize]);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Read the words file given on the command line
    let words = read_words(&args[1]);
    find_5words_dumb(&words);
}
