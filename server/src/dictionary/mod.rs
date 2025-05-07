use trie_rs::Trie;

#[allow(dead_code)]
mod words;

pub fn words_4() -> Trie<u8> {
    let mut trie = trie_rs::TrieBuilder::new();
    for w in words::words_4() {
        trie.push(w);
    }
    trie.build()
}
