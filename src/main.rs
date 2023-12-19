use base64::{engine::general_purpose, Engine as _};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn word_set(path: impl AsRef<Path>) -> Vec<char> {
    let data = fs::read_to_string(path).unwrap();
    data.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    let special = "。，：、！）（《》“”".chars();
    let hsk1 = word_set("HSK 1.txt");
    let hsk2 = word_set("HSK 2.txt");
    let hsk3 = word_set("HSK 3.txt");
    let hsk4 = word_set("HSK 4.txt");
    let hsk5 = word_set("HSK 5.txt");
    let hsk6 = word_set("HSK 6.txt");
    let good_set: HashSet<char> = HashSet::from_iter(
        hsk1.into_iter()
            .chain(hsk2.into_iter())
            // .chain(hsk3.into_iter())
            // .chain(hsk4.into_iter())
            .chain(special),
    );
    // let bad_set: HashSet<char> = HashSet::from_iter(
    //     hsk4.into_iter()
    //         .chain(hsk5.into_iter())
    //         .chain(hsk6.into_iter()),
    // );
    let tiktoken =
        fs::read_to_string("qwen.tiktoken").expect("Should have been able to read the file");
    let tokens = tiktoken
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(base64_token, token_id)| {
            Some((
                String::from_utf8(general_purpose::STANDARD.decode(base64_token).ok()?).ok()?,
                token_id,
            ))
        })
        .filter_map(|(token, token_id)| Some((token, i64::from_str(token_id).ok()?)));
    // .filter(|(token, _token_id)| token == " ");
    // .filter(|(token, _token_id)| token.chars().any(unicode_blocks::is_cjk))
    // .filter(|(token, _token_id)| {
    //     !token
    //         .chars()
    //         .filter(|c| unicode_blocks::is_cjk(*c))
    //         .all(|c| good_set.contains(&c))
    // });
    // .filter(|(token, _token_id)| {
    //     token
    //         .chars()
    //         .filter(|c| unicode_blocks::is_cjk(*c))
    //         .any(|c| bad_set.contains(&c))
    // });
    println!("220-10");
    for (token, token_id) in tokens {
        println!("{token:?} {token_id}");
        // println!("{token_id}-1000");
    }
    // println!("Read tiktoken file");
    // println!("Read HSK files");
    // println!("Find all Chinese tokens");
    // println!("Filter out tokens if they are in the accepted set");
}
