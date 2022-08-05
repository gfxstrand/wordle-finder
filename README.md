Wordle combination finder
=========================

This is my solution to the problem posed by Matt Parker in this YouTube
video:

[![ Can you find: five five-letter words with twenty-five unique letters? ](https://img.youtube.com/vi/_-AfhLQfb6w/0.jpg)](https://www.youtube.com/watch?v=_-AfhLQfb6w)

## Differences from Matt's approach

My approach is different from Matt's in 3 key ways

### 1. Rust

This should be obvious.  Mine's written in Rust which is a compiled
language and therefore usually a bit faster.

### 2. Clever use of bitsets

While thinking about this problem, I had the realization that likely part
of the expense Matt incurred was in checking for uniqueness of letters.
Naively, it's up to 25 comparisons to see if two 5-letter words share any
letters.  If you're clever and you sort the letters in each word first
(order doesn't matter), you can do it in at most 10 comparisons by doing a
sort of merge sort.

However, we can be even more clever!  Since there are only 26 letters in
the English alphabet and we don't care about upper vs. lower case, we can
represent a word as a 32-bit integer where each bit corresponds to a letter
with the lowest bit being "A", and the 26th bit being "Z".  By doing this,
we can check if a word has 5 unique letters with
`word.letters.count_ones() == 5` and we can check if two words share any
bits with `(a.letters & b.letters) != 0`.  While this doesn't improve the
algorithmic runtime at all it does reduce the constant factor a lot.

### 3a. Use a dumb loop

Matt was actually a bit too clever in his approach.  The idea of comparing
pairs of pairs looks good on the face, but it's actually worse than a
well-written quintuple loop.  Using Matt's original word list, with
anagrams removed, there are:

 - 5977 words with unique letters
 - 3213696 pairs of words with unique letters
 - 95866204 sets of three words with unique letters
 - 26133319 sets of four words with unique letters
 - 538 sets of five words with unique letters

Matt thought he was saving himself some effort looking at pairs of pairs
rather than doing something more naive.  However, this ends up to work out
worse than the naive solution.  Even if you're good about only considering
unique pairs (a naive double loop would see every pair twice), you still
end up looking at $(3,213,696 * 3,213,696) / 2 = 5,163,920,990,208$ pairs
of pairs.  If, on the other hand, you look at sets of three words and
compare those to the original set of single words, you end up looking at
$95,866,204 * 5,977 = 572,992,301,308$ sets of four words which is a whole
order of magnitude fewer.  Thanks to the reduction you get by throwing out
sets as quickly as you find a duplicate letter, building them up one
additional word at a time ends up considering far fewer sets in the end.

It's also implementable as a simple quintuple loop with no additional
storage so the only thing my implementation has to store is the original
list of words and my list of words converted to bitsets.  The intermediate
sets of 2, 3, 4, and 5 words are never stored, leading to what's likely a
much lower memory footprint.

### 3b. Use slightly cleverer dumb loop

While the dumb loop approach cuts out an order of magnitude, it's still way
slower than we want.  A lot of this comes from the fact that we keep trying
all sorts of word combinations we should know up-front won't work.  The new
approach is a bit of a hybrid.  First, we compute the list of all pairs that
have 10 unique letters.  At the same time, for each word, we record its range
in the list of pairs where that word matches the first element in the pair.

Armed with these pairs, we're able to do the final loop.  Instead of iterating
over all the words later in list like we did in the original dumb version, we
look up the range for the last word in our set and walk that subrange of the
list of pairs and look at the other word in the pair.  If you consider the
words as nodes in a graph with edges wherever two words share no letters, this
is the equivalent of walking only the edges coming out of a node rather than
all the nodes (words) in the graph.  This significantly reduces the runtime.
