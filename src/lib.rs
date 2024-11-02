use std::cmp::min;
use std::collections::HashMap;
use std::str::Chars;
mod dictionary;
pub use dictionary::get_dictionary;
use regex::Regex;

struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}

/// Struct is used when there are too many or too few characters in the input word
///
/// チェックする単語に文字の過不足があった場合に使用される構造体です
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterPositon {
    /// There is an over/under on the initial letter of the word(単語の頭文字に過不足があります)
    Head,
    /// There is an over/under at the end of a word(単語の末尾の文字に過不足があります)
    Tail,
}

/// Enum that classifies the type of typo
///
/// タイポの種類を分類する列挙型です
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypoType {
    /// Extra character in the check word(チェックする単語に余分な文字が入っている)
    ExtraCharacters {
        character: char,
        position: CharacterPositon,
    },
    /// Missing character in the check word(チェックする単語に足りない文字がある)
    MissingCharacters {
        character: char,
        position: CharacterPositon,
    },
    /// The check word and the correct word have a different character in close proximity in the Qwert sequence on the keyboard.(チェックする単語と正しい単語で違う文字がキーボードのQwert配列で近い位置にある)
    ///
    /// Ex. a => [q, w, s, x, z]
    CloseKeyboardPlacement,
    ///  The check word and the correct word are similar in shape.(チェックする単語と正しい単語で違う文字が形状として似ている)
    ///
    /// Ex. o => [a, c, e]
    SimilarShapes,
    /// Word that cannot be classified(分類ができない単語)
    UndefinedType,
}

/// Returns the name of the enumerator stored in the TypoType enumeration type.
/// When using this function, the fields of the ExtraCharacters and MissingCharacters are omitted.
///
/// TypoTypeの列挙型に格納されている列挙子の名前を返します。
/// このときExtraCharactersとMissingCharactersの構造体の中身は省略されます。
///
/// # Arguments
///
/// * `typo_type` - Words to check(列挙子名を取得したいタイポタイプ)
///
/// # Examples
///
/// ```
/// use typo_checker::TypoType;
/// use typo_checker::CharacterPositon;
/// use typo_checker::get_typo_type_name;
///
///
/// let typo_type = TypoType::ExtraCharacters{character: 'a', position: CharacterPositon::Head};
/// let typo_type_name = get_typo_type_name(&typo_type);
/// println!("typo_type_name: {:?}", typo_type_name);
/// ```
pub fn get_typo_type_name(typo_type: &TypoType) -> String {
    match typo_type {
        TypoType::ExtraCharacters { .. } => "ExtraCharacters".to_string(),
        TypoType::MissingCharacters { .. } => "MissingCharacters".to_string(),
        TypoType::CloseKeyboardPlacement => "CloseKeyboardPlacement".to_string(),
        TypoType::SimilarShapes => "SimilarShapes".to_string(),
        TypoType::UndefinedType => "UndefinedType".to_string(),
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
/// * `typo_type` - Type of typo(タイポの種類)
#[derive(Debug, Clone)]
pub struct SimilarWord {
    spelling: String,
    levenshtein_length: usize,
    typo_type: TypoType,
}

impl SimilarWord {
    pub fn new(spelling: String, levenshtein_length: usize) -> SimilarWord {
        SimilarWord {
            spelling,
            levenshtein_length,
            typo_type: TypoType::UndefinedType,
        }
    }

    fn sort_by_typo_type(
        similar_word_list: &mut Vec<SimilarWord>,
        sort_typo_type_setting: &Vec<TypoType>,
    ) {
        let typo_type_order: HashMap<String, usize> = sort_typo_type_setting
            .iter()
            .enumerate()
            .map(|(i, typo_type)| (get_typo_type_name(typo_type), i))
            .collect();

        similar_word_list.sort_by(|a, b| {
            let a_order = typo_type_order
                .get(&get_typo_type_name(&a.typo_type))
                .unwrap();
            let b_order = typo_type_order
                .get(&get_typo_type_name(&b.typo_type))
                .unwrap();
            a_order.cmp(b_order)
        });
    }
}

/// Struct to store typo search results.
///
/// タイポの検索結果を格納する構造体です
#[derive(Debug)]
pub struct TypoCheckResult {
    /// `match_word` - Stores the exact match(完全一致した単語を格納します)
    match_word: Option<String>,
    /// `similar_word_list` - Stores information on similar words in an array(似ている単語の情報を配列で格納します)
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
/// use typo_checker::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
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

/// When the check word is compared to the correct word, if there are excesses or deficiencies before or after the word, the typo_type of similar_word is changed to ExtraCharacters or MissingCharacters.
///
/// チェックする単語を正しい単語と比較したときに、単語の前後に過不足があればsimilar_wordのtypo_typeをExtraCharactersかMissingCharactersに変更します。
///
/// # Arguments
///
/// * `check_word` - The check word(チェックする単語)
/// * `similar_word` - SimilarWord type storing the correct word(正しい単語を格納したSimilarWord型)
///
/// # Examples
///
/// ```
/// use typo_checker::SimilarWord;
/// use typo_checker::find_missing_or_extra_chars;
///
/// let check_word = "applee";
/// let similar_word = SimilarWord::new("apple".to_string(), 1);
/// let return_word = find_missing_or_extra_chars(check_word, similar_word);
/// println!("return_word: {:?}", return_word);
/// ```
pub fn find_missing_or_extra_chars(check_word: &str, mut similar_word: SimilarWord) -> SimilarWord {
    let check_len = check_word.chars().count();
    let similar_len = similar_word.spelling.chars().count();

    if similar_len < check_len {
        // similar_wordが短い場合、check_wordに入っている余分な文字を探す
        let re_prefix =
            Regex::new(&format!(r"^{}(.+)", regex::escape(&similar_word.spelling))).unwrap();
        let re_suffix =
            Regex::new(&format!(r"(.+){}$", regex::escape(&similar_word.spelling))).unwrap();

        if let Some(captures) = re_prefix.captures(check_word) {
            let missing_prefix = captures.get(1).unwrap().as_str();
            similar_word.typo_type = TypoType::ExtraCharacters {
                character: missing_prefix.chars().next().unwrap(),
                position: CharacterPositon::Tail,
            };
        }

        if let Some(captures) = re_suffix.captures(check_word) {
            let missing_prefix = captures.get(1).unwrap().as_str();
            similar_word.typo_type = TypoType::ExtraCharacters {
                character: missing_prefix.chars().next().unwrap(),
                position: CharacterPositon::Head,
            };
        }
    } else {
        // similar_wordが長い場合、check_wordに足りない文字を探す
        let re_prefix = Regex::new(&format!(r"^(.+){}", regex::escape(check_word))).unwrap();
        let re_suffix = Regex::new(&format!(r"{}(.+)$", regex::escape(check_word))).unwrap();

        if let Some(captures) = re_prefix.captures(&similar_word.spelling) {
            let extra_prefix = captures.get(1).unwrap().as_str();
            similar_word.typo_type = TypoType::MissingCharacters {
                character: extra_prefix.chars().next().unwrap(),
                position: CharacterPositon::Head,
            };
        }

        if let Some(captures) = re_suffix.captures(&similar_word.spelling) {
            let extra_suffix = captures.get(1).unwrap().as_str();
            similar_word.typo_type = TypoType::MissingCharacters {
                character: extra_suffix.chars().next().unwrap(),
                position: CharacterPositon::Tail,
            };
        }
    }
    similar_word
}

/// Returns a hashmap of adjacent alphabets on a Qwert array keyboard.
///
/// Qwert配列のキーボードで隣接している単語のハッシュマップを返します。
///
/// # Examples
///
/// ```
/// use typo_checker::close_keyboard_placement_list;
///
/// let qwerty_hash_map = close_keyboard_placement_list();
/// println!("qwerty_hash_map: {:?}", qwerty_hash_map);
/// ```
pub fn close_keyboard_placement_list() -> HashMap<char, Vec<char>> {
    let mut output_hashmap: HashMap<char, Vec<char>> = HashMap::new();

    // キーボード1列目
    output_hashmap.insert('q', vec!['w', 's', 'a']);
    output_hashmap.insert('w', vec!['q', 'e', 'a', 's', 'd']);
    output_hashmap.insert('e', vec!['w', 'r', 's', 'd', 'f']);
    output_hashmap.insert('r', vec!['e', 't', 'd', 'f', 'g']);
    output_hashmap.insert('t', vec!['r', 'y', 'f', 'g', 'h']);
    output_hashmap.insert('y', vec!['t', 'u', 'g', 'h', 'j']);
    output_hashmap.insert('u', vec!['y', 'i', 'h', 'j', 'k']);
    output_hashmap.insert('i', vec!['u', 'o', 'j', 'k', 'l']);
    output_hashmap.insert('o', vec!['i', 'p', 'k', 'l']);
    output_hashmap.insert('p', vec!['o', 'l']);

    // キーボード2列目
    output_hashmap.insert('a', vec!['q', 'w', 's', 'x', 'z']);
    output_hashmap.insert('s', vec!['q', 'w', 'e', 'd', 'c', 'x', 'z', 'a']);
    output_hashmap.insert('d', vec!['w', 'e', 'r', 'f', 'v', 'c', 'x', 's']);
    output_hashmap.insert('f', vec!['e', 'r', 't', 'g', 'b', 'v', 'c', 'd']);
    output_hashmap.insert('g', vec!['r', 't', 'y', 'h', 'n', 'b', 'v', 'f']);
    output_hashmap.insert('h', vec!['t', 'y', 'u', 'j', 'm', 'n', 'b', 'g']);
    output_hashmap.insert('j', vec!['y', 'u', 'i', 'k', 'm', 'n', 'h']);
    output_hashmap.insert('k', vec!['u', 'i', 'o', 'l', 'm', 'j']);
    output_hashmap.insert('l', vec!['i', 'o', 'p', 'k']);

    // キーボード3列目
    output_hashmap.insert('z', vec!['a', 's', 'x']);
    output_hashmap.insert('x', vec!['a', 's', 'd', 'c', 'z']);
    output_hashmap.insert('c', vec!['s', 'd', 'f', 'v', 'x']);
    output_hashmap.insert('v', vec!['d', 'f', 'g', 'b', 'c']);
    output_hashmap.insert('b', vec!['f', 'g', 'h', 'n', 'v']);
    output_hashmap.insert('n', vec!['g', 'h', 'j', 'm', 'b']);
    output_hashmap.insert('m', vec!['h', 'j', 'k', 'n']);

    output_hashmap
}

/// Returns an array of groups of alphabets that are similar in shape.
/// Alphabets in the same array are considered “similar in shape”.
///
/// 形状が似ているアルファベットのグループの配列を返します。
/// 同じ配列に入っているアルファベットは「形状が似ている」と見做しています。
///
/// # Arguments
///
/// * `check_word` - The check word(チェックする単語)
/// * `similar_word` - SimilarWord type storing the correct word(正しい単語を格納したSimilarWord型)
///
/// # Examples
///
/// ```
/// use typo_checker::similar_shape_list;
///
/// let similar_group = similar_shape_list();
/// println!("similar_group: {:?}", similar_group);
/// ```
pub fn similar_shape_list() -> Vec<Vec<char>> {
    let mut output_vec: Vec<Vec<char>> = Vec::new();

    output_vec.push(vec!['a', 'c', 'e', 'o']);
    output_vec.push(vec!['b', 'd']);
    output_vec.push(vec!['f', 'l']);
    output_vec.push(vec!['g', 'q']);
    output_vec.push(vec!['m', 'n']);
    output_vec.push(vec!['p', 'q']);
    output_vec.push(vec!['u', 'v']);

    output_vec
}

/// Change the typo_type of similar_word to SimilarShapes or CloseKeyboardPlacement when one different character has a similar shape for the same string of characters.
/// ※In this library, check_word and temp_word to be put into this function are “with Levenshtein distance of 1”, so there is always one different character.
///
/// 同じ文字数の文字列に対して、異なる1文字が形状が似ていたときにtemp_wordのtypo_typeをSimilarShapesかCloseKeyboardPlacementに変更します。
/// ※このライブラリではこの関数に入れるcheck_wordとtemp_wordは「レーベンシュタイン距離が1のもの」であるため、必ず1文字違う文字が存在しています。
///
/// # Arguments
///
/// * `check_word` - The check word(チェックする単語)
/// * `temp_word` - SimilarWord type storing the correct word(正しい単語を格納したSimilarWord型)
///
/// # Examples
///
/// ```
/// use typo_checker::SimilarWord;
/// use typo_checker::find_different_a_char;
///
/// let check_word = "applo";
/// let temp_word = SimilarWord::new("apple".to_string(), 1);
/// let return_word = find_different_a_char(check_word, temp_word);
/// println!("return_word: {:?}", return_word);
/// ```
pub fn find_different_a_char(check_word: &str, mut temp_word: SimilarWord) -> SimilarWord {
    let similar_shape = similar_shape_list();
    let close_keyboard_placement = close_keyboard_placement_list();

    for (c, t) in check_word.chars().zip(temp_word.spelling.chars()) {
        if c != t {
            //形状が似ているか確認
            for tmp_similar_char in similar_shape.iter() {
                if tmp_similar_char.contains(&c) && tmp_similar_char.contains(&t) {
                    temp_word.typo_type = TypoType::SimilarShapes;
                    return temp_word;
                }
            }

            //キーボード配置が近いか確認
            let pickup_close_keyboard_placement_vec = close_keyboard_placement.get(&c).unwrap();

            if pickup_close_keyboard_placement_vec.contains(&t) {
                temp_word.typo_type = TypoType::CloseKeyboardPlacement;
            }
        }
    }
    temp_word
}

/// Returns typo-check results for the check word based on output criteria such as the number of pieces to output and sort order.
///
/// 出力する個数やソートの順序などの出力条件に基づいて、単語のタイポチェック結果を返します。
///
/// # Arguments
///
/// * `check_word` - The check word(チェックする単語)
/// * `check_word_length` - Length of the check word(チェックする単語の文字数)
/// * `similar_word_list` - List of words similar to the check word(チェックする単語に似ている単語のリスト)
/// * `output_levenshtein_cutoff` - Cutoff values by Levenshtein distance for output list(出力する似ている単語リストのレーベンシュタイン距離によるカットオフ数値)
/// * `pickup_similar_word_num` - Cutoff value for the number of elements in output list(出力する似ている単語リストの要素数のカットオフ数値)
/// * `sort_order_of_typo_type` - Sort criteria by TypoType for output list(出力する似ている単語リストのTypoTypeによるソート条件)
fn get_top_similar_words(
    check_word: String,
    check_word_length: usize,
    mut similar_word_list: Vec<SimilarWord>,
    output_levenshtein_cutoff: Option<usize>,
    pickup_similar_word_num: usize,
    sort_order_of_typo_type: Option<&Vec<TypoType>>,
) -> Vec<SimilarWord> {
    // `levenshtein_length` の小さい順にソート
    similar_word_list.sort_by_key(|word| word.levenshtein_length);

    // カットオフが指定されている場合、それより文字数が多い単語をフィルタする
    if let Some(cutoff) = output_levenshtein_cutoff {
        similar_word_list.retain(|word| word.levenshtein_length <= cutoff);
    }

    // カットオフが1のものについてTypoTypeの判別を行う
    for temp_word in similar_word_list.iter_mut() {
        if temp_word.levenshtein_length == 1 {
            //チェックする単語との文字数の比較を行う
            if check_word_length == temp_word.spelling.chars().count() {
                // CloseKeyboardPlacementかSimilarShapesの判別を行う
                *temp_word = find_different_a_char(&check_word, temp_word.clone())
            } else {
                // MissingCharactersの処理を行う
                *temp_word = find_missing_or_extra_chars(&check_word, temp_word.clone());
            }
        } else {
            continue;
        }
    }

    // TypoTypeに応じてソートを実行する
    let default_sort_typo_type = vec![
        TypoType::ExtraCharacters {
            character: 'A',
            position: CharacterPositon::Head,
        },
        TypoType::MissingCharacters {
            character: 'Z',
            position: CharacterPositon::Tail,
        },
        TypoType::SimilarShapes,
        TypoType::CloseKeyboardPlacement,
        TypoType::UndefinedType,
    ];

    let sort_typo_type = sort_order_of_typo_type.unwrap_or(&default_sort_typo_type);
    SimilarWord::sort_by_typo_type(&mut similar_word_list, &sort_typo_type);

    // 結果が必要な数以下の場合、そのまま返す
    if similar_word_list.len() <= pickup_similar_word_num {
        similar_word_list
    } else {
        // 必要な数までを取り出して返す
        similar_word_list
            .into_iter()
            .take(pickup_similar_word_num)
            .collect()
    }
}

/// Returns TypoCheckResult type words that match or are similar to the word to be checked.
/// Similar_word_list of type TypoCheckResult contains the top `pickup_similar_word_num` words with Levenshtein distance(less than or equal to `output_levenshtein_cutoff`).
///
/// チェックする単語に合致、もしくは類似する単語をTypoCheckResult型で返却します。
/// TypoCheckResult型のsimilar_word_listには、レーベンシュタイン距離がoutput_levenshtein_cutoff以下&pickup_similar_word_numで指定した個数の上位の単語が格納されます。
///
/// # Arguments
///
/// * `check_word` - Words to check(チェックする単語)
/// * `output_levenshtein_cutoff` - Cutoff value of Levenshtein distance to output(出力するレーベンシュタイン距離のカットオフ値)
/// * `pickup_similar_word_num` - Number of words to store in the list of similar_word_list(似ている単語のリストに格納する単語数)
/// * `sort_order_of_typo_type` - Sort criteria by TypoType for output list(出力する似ている単語リストのTypoTypeによるソート条件)
///
/// # Examples
///
/// ```
/// use typo_checker::TypoType;
/// use typo_checker::CharacterPositon;
///
/// let check_word = "applo";
/// let custom_sort_order = vec![TypoType::SimilarShapes, TypoType::CloseKeyboardPlacement, TypoType::UndefinedType, TypoType::ExtraCharacters { character: 'A', position: CharacterPositon::Head, }, TypoType::MissingCharacters { character: 'Z', position: CharacterPositon::Tail, }, ];
/// let typo_chec_result = typo_checker::check_a_word(check_word.to_string(), Some(3), 20, Some(&custom_sort_order));
/// println!("typo_chec_result: {:?}", typo_chec_result);
/// ```
pub fn check_a_word(
    check_word: String,
    output_levenshtein_cutoff: Option<usize>,
    pickup_similar_word_num: usize,
    sort_order_of_typo_type: Option<&Vec<TypoType>>,
) -> TypoCheckResult {
    let lowercase_check_word = check_word.to_lowercase();
    let check_word_length = lowercase_check_word.chars().count();
    let select_word_range: usize = match output_levenshtein_cutoff {
        Some(range_num) => {
            if range_num == 1 {
                panic!("Please select output_levenshtein_cutoff > 1 !!");
            } else {
                range_num
            }
        }
        None => 2,
    };

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
        lowercase_check_word,
        check_word_length,
        similar_word_list,
        output_levenshtein_cutoff,
        pickup_similar_word_num,
        sort_order_of_typo_type,
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_or_extra_chars_head() {
        // Head のテストケース
        let check_word = "ello";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(
            result.typo_type,
            TypoType::MissingCharacters {
                character: 'h',
                position: CharacterPositon::Head
            }
        );
    }

    #[test]
    fn test_find_missing_or_extra_chars_tail() {
        // Tail のテストケース
        let check_word = "hell";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(
            result.typo_type,
            TypoType::MissingCharacters {
                character: 'o',
                position: CharacterPositon::Tail
            }
        );
    }

    #[test]
    fn test_find_extra_chars_head() {
        // Head の余分な文字テストケース
        let check_word = "ahello";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(
            result.typo_type,
            TypoType::ExtraCharacters {
                character: 'a',
                position: CharacterPositon::Head
            }
        );
    }

    #[test]
    fn test_find_extra_chars_tail() {
        // Tail の余分な文字テストケース
        let check_word = "helloo";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(
            result.typo_type,
            TypoType::ExtraCharacters {
                character: 'o',
                position: CharacterPositon::Tail
            }
        );
    }

    #[test]
    fn test_find_typo_type_none() {
        // 正しい単語の場合のテストケース
        let check_word = "hello";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(result.typo_type, TypoType::UndefinedType);
    }

    #[test]
    fn test_find_multiple_missing_chars() {
        // 複数の文字が足りない場合のテストケース
        let check_word = "hlo";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(result.typo_type, TypoType::UndefinedType);
    }

    #[test]
    fn test_find_multiple_extra_chars() {
        // 複数の文字が余分な場合のテストケース
        let check_word = "heelllo";
        let similar_word = SimilarWord::new("hello".to_string(), 1);
        let result = find_missing_or_extra_chars(check_word, similar_word);

        assert_eq!(result.typo_type, TypoType::UndefinedType);
    }

    #[test]
    fn test_find_different_a_char_similar_shapes() {
        let check_word = "cot";
        let temp_word = SimilarWord::new("cat".to_string(), 1);
        let result = find_different_a_char(check_word, temp_word);

        if let TypoType::SimilarShapes = result.typo_type {
            // テストが通れば成功
        } else {
            panic!(
                "Expected TypoType::SimilarShapes but got {:?}",
                result.typo_type
            );
        }
    }

    #[test]
    fn test_find_different_a_char_close_keyboard_placement() {
        let check_word = "try".to_string();
        let similar_word = SimilarWord {
            spelling: "trt".to_string(), // "y" -> "t" は隣接キーだが SimilarShapes には該当しない
            levenshtein_length: 1,
            typo_type: TypoType::UndefinedType,
        };

        // `find_different_a_char`関数を呼び出して、誤りのタイプを判別
        let result = find_different_a_char(&check_word, similar_word);

        // `TypoType::CloseKeyboardPlacement` が設定されているか確認
        assert!(matches!(result.typo_type, TypoType::CloseKeyboardPlacement));
    }

    #[test]
    fn test_find_different_a_char_no_typo_detected() {
        let check_word = "hoxe";
        let temp_word = SimilarWord::new("home".to_string(), 0);
        let result = find_different_a_char(check_word, temp_word);

        if let TypoType::UndefinedType = result.typo_type {
            // テストが通れば成功
        } else {
            panic!(
                "Expected TypoType::UndefinedType but got {:?}",
                result.typo_type
            );
        }
    }

    #[test]
    fn test_get_top_similar_words_default_typo_type_sorting() {
        let check_word = "tets".to_string();
        let check_word_length = check_word.len();
        let similar_word_list = vec![
            SimilarWord {
                spelling: "test".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::UndefinedType,
            },
            SimilarWord {
                spelling: "tsts".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::CloseKeyboardPlacement,
            },
            SimilarWord {
                spelling: "tots".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::SimilarShapes,
            },
            SimilarWord {
                spelling: "ttets".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::ExtraCharacters {
                    character: 's',
                    position: CharacterPositon::Head,
                },
            },
            SimilarWord {
                spelling: "tetss".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::ExtraCharacters {
                    character: 's',
                    position: CharacterPositon::Tail,
                },
            },
            SimilarWord {
                spelling: "ets".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::MissingCharacters {
                    character: 't',
                    position: CharacterPositon::Head,
                },
            },
            SimilarWord {
                spelling: "tet".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::MissingCharacters {
                    character: 's',
                    position: CharacterPositon::Tail,
                },
            },
        ];

        let result = get_top_similar_words(
            check_word,
            check_word_length,
            similar_word_list,
            None,
            7,
            None,
        );

        // デフォルトの並び順: ExtraCharacters -> MissingCharacters -> SimilarShapes -> CloseKeyboardPlacement -> UndefinedType
        assert_eq!(result.len(), 7);
        assert!(matches!(
            result[0].typo_type,
            TypoType::ExtraCharacters { .. }
        ));
        assert!(matches!(
            result[1].typo_type,
            TypoType::ExtraCharacters { .. }
        ));
        assert!(matches!(
            result[2].typo_type,
            TypoType::MissingCharacters { .. }
        ));
        assert!(matches!(
            result[3].typo_type,
            TypoType::MissingCharacters { .. }
        ));
        assert!(matches!(result[4].typo_type, TypoType::SimilarShapes));
        assert!(matches!(
            result[5].typo_type,
            TypoType::CloseKeyboardPlacement
        ));
        assert!(matches!(result[6].typo_type, TypoType::UndefinedType));
    }

    #[test]
    fn test_get_top_similar_words_basic_sorting() {
        let check_word = "test".to_string();
        let check_word_length = check_word.len();
        let similar_word_list = vec![
            SimilarWord::new("best".to_string(), 1),
            SimilarWord::new("tost".to_string(), 1),
            SimilarWord::new("toast".to_string(), 2),
        ];

        let result = get_top_similar_words(
            check_word,
            check_word_length,
            similar_word_list,
            None,
            2,
            None,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].spelling, "tost");
        assert_eq!(result[1].spelling, "best");
    }

    #[test]
    fn test_get_top_similar_words_with_cutoff() {
        let check_word = "test".to_string();
        let check_word_length = check_word.len();
        let similar_word_list = vec![
            SimilarWord::new("tost".to_string(), 1),
            SimilarWord::new("toast".to_string(), 2),
            SimilarWord::new("tasteo".to_string(), 3),
        ];

        let result = get_top_similar_words(
            check_word,
            check_word_length,
            similar_word_list,
            Some(2),
            3,
            None,
        );

        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|w| w.levenshtein_length <= 2));
    }

    #[test]
    fn test_get_top_similar_words_typo_type_sorting() {
        let check_word = "tets".to_string();
        let check_word_length = check_word.len();
        let similar_word_list = vec![
            SimilarWord {
                spelling: "test".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::UndefinedType,
            },
            SimilarWord {
                spelling: "tsts".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::CloseKeyboardPlacement,
            },
            SimilarWord {
                spelling: "tots".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::SimilarShapes,
            },
            SimilarWord {
                spelling: "ttets".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::ExtraCharacters {
                    character: 's',
                    position: CharacterPositon::Head,
                },
            },
            SimilarWord {
                spelling: "tetss".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::ExtraCharacters {
                    character: 's',
                    position: CharacterPositon::Tail,
                },
            },
            SimilarWord {
                spelling: "ets".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::MissingCharacters {
                    character: 't',
                    position: CharacterPositon::Head,
                },
            },
            SimilarWord {
                spelling: "tet".to_string(),
                levenshtein_length: 1,
                typo_type: TypoType::MissingCharacters {
                    character: 's',
                    position: CharacterPositon::Tail,
                },
            },
        ];

        let custom_sort_order = vec![
            TypoType::SimilarShapes,
            TypoType::CloseKeyboardPlacement,
            TypoType::UndefinedType,
            TypoType::ExtraCharacters {
                character: 'A',
                position: CharacterPositon::Head,
            },
            TypoType::MissingCharacters {
                character: 'Z',
                position: CharacterPositon::Tail,
            },
        ];

        let result = get_top_similar_words(
            check_word,
            check_word_length,
            similar_word_list,
            None,
            7,
            Some(&custom_sort_order),
        );

        assert_eq!(result.len(), 7);
        assert!(matches!(result[0].typo_type, TypoType::SimilarShapes));
        assert!(matches!(
            result[1].typo_type,
            TypoType::CloseKeyboardPlacement
        ));
        assert!(matches!(result[2].typo_type, TypoType::UndefinedType));
        assert!(matches!(
            result[3].typo_type,
            TypoType::ExtraCharacters { .. }
        ));
        assert!(matches!(
            result[4].typo_type,
            TypoType::ExtraCharacters { .. }
        ));
        assert!(matches!(
            result[5].typo_type,
            TypoType::MissingCharacters { .. }
        ));
        assert!(matches!(
            result[6].typo_type,
            TypoType::MissingCharacters { .. }
        ));
    }

    #[test]
    fn test_get_top_similar_words_limit_results() {
        let check_word = "tets".to_string();
        let check_word_length = check_word.len();
        let similar_word_list = vec![
            SimilarWord::new("tost".to_string(), 1),
            SimilarWord::new("tetsaa".to_string(), 2),
            SimilarWord::new("tetsaao".to_string(), 2),
        ];

        let result = get_top_similar_words(
            check_word,
            check_word_length,
            similar_word_list,
            None,
            1,
            None,
        );

        assert_eq!(result.len(), 1);
    }
}
