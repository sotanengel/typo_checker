use std::cmp::min;
use std::str::Chars;
mod dictionary;
pub use dictionary::get_dictionary;

struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}

/// Struct that stores information about similar word
///
/// 似ている単語の情報を格納する構造体です
///
/// # Arguments
///
/// * `spelling` - Spelling of similar words(似ている単語のスペル)
/// * `levenshtein_length` - Levenshtein Distance(レーベンシュタイン距離)
#[derive(Debug, Clone)]
pub struct SimilarWord {
    spelling: String,
    levenshtein_length: usize,
}

impl SimilarWord {
    fn new(spelling: String, levenshtein_length: usize) -> SimilarWord {
        SimilarWord {
            spelling,
            levenshtein_length,
        }
    }
}

/// Struct to store typo search results.
///
/// タイポの検索結果を格納する構造体です
///
/// # Arguments
///
/// * `match_word` - Stores the exact match(完全一致した単語を格納します)
/// * `similar_word_list` - Stores information on similar words in an array(似ている単語の情報を配列で格納します)
///
/// # Examples
///
/// ```
/// let a = "applo";
/// let typo_chec_result = typo_checker::check_a_word(a.to_string());
/// println!("typo_chec_result: {:?}", typo_chec_result);
/// ```
#[derive(Debug)]
pub struct TypoCheckResult {
    match_word: Option<String>,
    similar_word_list: Option<Vec<SimilarWord>>,
}

impl TypoCheckResult {
    fn new() -> TypoCheckResult {
        TypoCheckResult {
            match_word: None,
            similar_word_list: None,
        }
    }

    pub fn get_match_word(&self) -> String {
        if let Some(ref word) = self.match_word {
            word.clone()
        } else {
            "There is not match word".to_string()
        }
    }

    pub fn get_similar_word_list(&self) -> Vec<SimilarWord> {
        if let Some(ref word_list) = self.similar_word_list {
            word_list.to_vec()
        } else {
            Vec::new() // エラーメッセージの代わりに空のVecを返す
        }
    }
}

/// Calculate the Levenshtein distance
///
/// レーベンシュタイン距離を計算します
fn generic_levenshtein<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> usize
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let b_len = b.into_iter().count();

    let mut cache: Vec<usize> = (1..b_len + 1).collect();

    let mut result = b_len;

    for (i, a_elem) in a.into_iter().enumerate() {
        result = i + 1;
        let mut distance_b = i;

        for (j, b_elem) in b.into_iter().enumerate() {
            let cost = usize::from(a_elem != b_elem);
            let distance_a = distance_b + cost;
            distance_b = cache[j];
            result = min(result + 1, min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }

    result
}

/// Call generic_levenshtein to calculate the Levenshtein distance
///
/// レーベンシュタイン距離を計算するgeneric_levenshteinを呼び出します
///
/// # Arguments
///
/// * `a` - Word A to be compared(比較対象の単語A)
/// * `b` - Word B to be compared(比較対象の単語B)
///
/// # Examples
///
/// ```
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}

fn calculate_word_list_levenshtein_length(
    word_list: &[[Option<&str>; 5416]],
    check_word: &String,
    mut similar_word_list: Vec<SimilarWord>,
) -> Vec<SimilarWord> {
    for temp_same_length_word_list in word_list.iter() {
        for temp_word in temp_same_length_word_list.iter() {
            match temp_word {
                Some(word) => {
                    let levenshtein_length = levenshtein(&check_word, &word);
                    similar_word_list.push(SimilarWord::new(word.to_string(), levenshtein_length));
                }
                None => break,
            }
        }
    }
    similar_word_list
}

fn get_top_similar_words(
    mut similar_word_list: Vec<SimilarWord>,
    pickup_similar_word_num: usize,
) -> Vec<SimilarWord> {
    // `levenshtein_length`の小さい順にソート
    similar_word_list.sort_by_key(|word| word.levenshtein_length);

    if similar_word_list.len() <= pickup_similar_word_num {
        similar_word_list
    } else {
        similar_word_list
            .into_iter()
            .take(pickup_similar_word_num)
            .collect()
    }
}

/// Returns TypoCheckResult type words that match or are similar to the word to be checked.
/// Similar_word_list of type TypoCheckResult contains the top 5 words with short Levenshtein distance.
///
/// チェックする単語に合致、もしくは類似する単語をTypoCheckResult型で返却します。
/// TypoCheckResult型のsimilar_word_listには、レーベンシュタイン距離が短い&上位5つの単語が格納されます。
///
/// # Arguments
///
/// * `check_word` - Words to check(チェックする単語)
///
/// # Examples
///
/// ```
/// let a = "applo";
/// let typo_chec_result = typo_checker::check_a_word(a.to_string());
/// println!("typo_chec_result: {:?}", typo_chec_result);
/// ```
pub fn check_a_word(check_word: String) -> TypoCheckResult {
    let lowercase_check_word = check_word.to_lowercase();
    let check_word_length = lowercase_check_word.chars().count();
    let select_word_range: usize = 2;
    let pickup_similar_word_num: usize = 5;
    let word_dic = get_dictionary();

    let mut output = TypoCheckResult::new();
    let mut similar_word_list: Vec<SimilarWord> = Vec::new();

    // インデックスを初期化
    let mut select_word_upper_index: usize = 10;
    let mut select_word_lower_index: isize = 0; // isizeにして一時的に負の値も扱えるようにする

    // 文字数に応じたインデックスの計算
    if check_word_length == 1 {
        return output;
    } else if check_word_length == 2 {
        select_word_upper_index = (check_word_length - 2) + select_word_range;
        select_word_lower_index = (check_word_length - 2) as isize;
    } else if check_word_length == 21 {
        select_word_upper_index = check_word_length - 2;
        select_word_lower_index = (check_word_length - 2) as isize - select_word_range as isize;
    } else {
        select_word_upper_index = (check_word_length - 2) + select_word_range;
        select_word_lower_index = (check_word_length - 2) as isize - select_word_range as isize;
    }

    // インデックス範囲を調整
    select_word_lower_index = select_word_lower_index.max(0); // 下限は0にする
    select_word_upper_index = select_word_upper_index.min(word_dic.len()); // 上限はword_dicの長さにする

    let same_length_word_dic = &word_dic[check_word_length - 2];
    let selected_lower_word_dic =
        &word_dic[select_word_lower_index as usize..check_word_length - 2]; // isizeをusizeにキャスト
    let selected_upper_word_dic = &word_dic[check_word_length - 1..select_word_upper_index];

    // 完全に一致する単語を探索する
    for temp_word in same_length_word_dic.iter() {
        match temp_word {
            Some(word) => {
                let levenshtein_length = levenshtein(&lowercase_check_word, &word);

                if levenshtein_length == 0 {
                    output.match_word = Some(word.to_string());
                    output.similar_word_list = None;
                    return output;
                } else {
                    similar_word_list.push(SimilarWord::new(word.to_string(), levenshtein_length));
                }
            }
            None => break,
        };
    }

    // 類似する単語を探す(探す単語よりも文字数がselect_word_range少ないもの)
    similar_word_list = calculate_word_list_levenshtein_length(
        selected_lower_word_dic,
        &lowercase_check_word,
        similar_word_list,
    );

    // 類似する単語を探す(探す単語よりも文字数がselect_word_range多いもの)
    similar_word_list = calculate_word_list_levenshtein_length(
        selected_upper_word_dic,
        &lowercase_check_word,
        similar_word_list,
    );

    output.similar_word_list = Some(get_top_similar_words(
        similar_word_list,
        pickup_similar_word_num,
    ));

    output
}
