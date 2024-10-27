# typo_checker

> **Returns TypoCheckResult type words that match or are similar to the word to be checked**
> **チェックする単語に合致、もしくは類似する単語をTypoCheckResult型で返却します**

```
fn main() {
    let a = "typo";
    let typo_chec_result = typo_checker::check_a_word(a.to_string());

    println!("typo_chec_result: {:?}", typo_chec_result);
}
```

Output: typo_chec_result: TypoCheckResult { match_word: None, similar_word_list: Some([SimilarWord { spelling: "hypo", levenshtein_length: 1 }, SimilarWord { spelling: "type", levenshtein_length: 1 }, SimilarWord { spelling: "Expo", levenshtein_length: 2 }, SimilarWord { spelling: "hype", levenshtein_length: 2 }, SimilarWord { spelling: "taco", levenshtein_length: 2 }]) }

[Crates.io]: https://crates.io/crates/typo_checker
[Documentation]: https://docs.rs/typo_checker/0.1.0/typo_checker/
