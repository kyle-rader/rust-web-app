use automata::dictionary::words_4;
use clap::Parser;
use trie_rs::Trie;

#[derive(Debug, Parser)]
#[clap(name = "tri", version, about)]
struct Cli {
    /// Play a game of tri
    query: String,
}

fn main() -> anyhow::Result<()> {
    let words_4 = words_4();

    let Cli { query } = Cli::parse();
    println!("Search for: {query}");
    search(query, &words_4)?;

    Ok(())
}

fn search(query: String, tri: &Trie<u8>) -> anyhow::Result<()> {
    for r in tri.predictive_search(query).collect::<Vec<String>>() {
        println!("{r}");
    }
    Ok(())
}
