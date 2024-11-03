# typo_checker

> **Returns TypoCheckResult type words that match or are similar to the word to be checked**
> **チェックする単語に合致、もしくは類似する単語をTypoCheckResult型で返却します**

Code:

```
fn main() {
    let a = "typ";
    let typo_chec_result = typo_checker::check_a_word(a.to_string(), Some(2), 5, None);

    println!("typo_chec_result: {:?}", typo_chec_result);
}
```

Output:

```
typo_chec_result: TypoCheckResult { match_word: None, similar_word_list: Some([SimilarWord { spelling: "type", levenshtein_length: 1, typo_type: MissingCharacters { character: 'e', position: Tail } }, SimilarWord { spelling: "gyp", levenshtein_length: 1, typo_type: CloseKeyboardPlacement }, SimilarWord { spelling: "tup", levenshtein_length: 1, typo_type: CloseKeyboardPlacement }, SimilarWord { spelling: "tap", levenshtein_length: 1, typo_type: UndefinedType }, SimilarWord { spelling: "tip", levenshtein_length: 1, typo_type: UndefinedType }]) }
```

[Crates.io](https://crates.io/crates/typo_checker)

[Documentation](https://docs.rs/typo_checker/1.0.0/typo_checker/)
