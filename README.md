![](https://github.com/Narsil/trie-rs/workflows/build/badge.svg)

# trie-rs

Very simple trie implementation to have fast implementations of:
 - `common_prefix_search`: give every item in the trie that is
 a prefix of the query
 - `search`: give every item in the trie that would follow the query
 In order for search to be fast, an index has to be built, which can
 become very large if you intend to store many items in the trie.
 PR welcomes to store partial indexes for large tries.

```rust
use trie_rs::TrieBuilder;

let build_index = true;
let mut builder = TrieBuilder::new(build_index);
builder.push(&vec!['A', 'l', 'a', 'b', 'a', 'm', 'a']);
builder.push(&vec!['A', 'l', 'a', 's', 'k', 'a']);
builder.push(&vec!['A', 'l', 'a', 's']);
let trie = builder.build();
assert_eq!(trie.search(&vec!['A', 'l', 'a', 's']).unwrap(),
&vec![
    vec!['A', 'l', 'a', 's'],
    vec!['A', 'l', 'a', 's', 'k', 'a'],
]);
assert_eq!(trie.common_prefix_search(&vec!['A', 'l', 'a', 's', 'k', 'a']),
vec![
    vec!['A', 'l', 'a', 's'],
    vec!['A', 'l', 'a', 's', 'k', 'a'],
]);
```

The item stored in the Trie needs eq + Hash as under the hood we use
a hashmap for fast query. We also need copy because most of the time
we will use very small items as trie elements, like `char` or `u8` for
strings, or ints.

