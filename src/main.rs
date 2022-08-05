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
    word: String,
    letters: u32,
}

fn find_sets_of_5_dumb_loop(words: &Vec<Word>) {
    let num_words = words.len();

    let mut num_sets_2: u32 = 0;
    let mut num_sets_3: u32 = 0;
    let mut num_sets_4: u32 = 0;
    let mut num_sets_5: u32 = 0;

    for a in 0..num_words {
        let wa = &words[a];
        let letters1 = wa.letters;
        for b in (a+1)..num_words {
            let wb = &words[b];

            if (letters1 & wb.letters) != 0 {
                continue;
            }
            num_sets_2 += 1;
            let letters2 = letters1 | wb.letters;

            for c in (b+1)..num_words {
                let wc = &words[c];

                if (letters2 & wc.letters) != 0 {
                    continue;
                }
                num_sets_3 += 1;
                let letters3 = letters2 | wc.letters;

                for d in (c+1)..num_words {
                    let wd = &words[d];

                    if (letters3 & wd.letters) != 0 {
                        continue;
                    }
                    num_sets_4 += 1;
                    let letters4 = letters3 | wd.letters;

                    for e in (d+1)..num_words {
                        let we = &words[e];

                        if (letters4 & we.letters) != 0 {
                            continue;
                        }
                        num_sets_5 += 1;

                        println!("{}, {}, {}, {}, {}",
                                 wa.word, wb.word, wc.word, wd.word, we.word);
                    }
                }
            }
        }
    }
    println!("Found {} words with unique letters", num_words);
    println!("Found {} pairs of words with unique letters", num_sets_2);
    println!("Found {} sets of three words with unique letters", num_sets_3);
    println!("Found {} sets of four words with unique letters", num_sets_4);
    println!("Found {} sets of five words with unique letters", num_sets_5);
}

fn main() {
    // Open the file name given as a command line argument
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap();

    let mut words: Vec<Word> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let word = line.unwrap().trim().to_string();
        if word.len() != 5 {
            continue;
        }

        let mut letters = 0u32;
        for c in word.to_uppercase().as_bytes() {
            assert!(b'A' <= *c && *c <= b'Z');
            letters |= 1u32 << (*c - b'A');
        }
        if letters.count_ones() != 5 {
            continue;
        }

        words.push(Word {
            word: word,
            letters: letters,
        });
    }

    // Get rid of any annograms.  This isn't necessary but cuts down on
    // runtime and lets us verify results against Matt's algorithm
    words.sort_by_key(|w| w.letters.reverse_bits());
    words.dedup_by_key(|w| w.letters);

    find_sets_of_5_dumb_loop(&words);
}
