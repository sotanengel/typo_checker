use std::str::Chars;
use std::cmp::min;
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
#[derive(Debug)]
struct SimilarWord {
    spelling: String,
    levenshtein_length: usize,
}

impl SimilarWord {
    fn new(spelling: String, levenshtein_length: usize) -> SimilarWord {
        SimilarWord{
          spelling,
          levenshtein_length,
        }
    }
}

#[derive(Debug)]
pub struct TypoChecResult {
    match_word: Option<String>,
    similar_word_list: Option<Vec<SimilarWord>>,
}

impl TypoChecResult {
    fn new() -> TypoChecResult {
        TypoChecResult {
          match_word: None,
          similar_word_list: None,
        }
    }

    fn get_match_word(&self) -> String {
        if let Some(ref word) = self.match_word {
            word.clone()
        } else {
            "There is not match word".to_string()
        }
    }
}

/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one sequence into the other.
///
/// ```
/// use strsim::generic_levenshtein;
///
/// assert_eq!(3, generic_levenshtein(&[1,2,3], &[1,2,3,4,5,6]));
/// ```
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

/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one string into the other.
///
/// ```
/// use strsim::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}

fn calculate_word_list_levenshtein_length(word_list: &[[Option<&str>; 5447]], check_word: &String, mut similar_word_list: Vec<SimilarWord>) -> Vec<SimilarWord> {
  for temp_same_length_word_list in word_list.iter() {
    for temp_word in temp_same_length_word_list.iter(){
      match temp_word {
        Some(word) => {
          let levenshtein_length = levenshtein(&check_word, &word);
          similar_word_list.push(SimilarWord::new(word.to_string(), levenshtein_length));
        },
        None => break,
      }
    }
  }
  similar_word_list
}

fn get_top_similar_words(mut similar_word_list: Vec<SimilarWord>, pickup_similar_word_num: usize) -> Vec<SimilarWord> {
  // `levenshtein_length`の小さい順にソート
  similar_word_list.sort_by_key(|word| word.levenshtein_length);

  if similar_word_list.len()<=pickup_similar_word_num{
    similar_word_list
  } else {
    similar_word_list.into_iter().take(pickup_similar_word_num).collect()
  }
}

pub fn check(check_word: String) -> TypoChecResult{
  let check_word_length = check_word.chars().count();
  let select_word_range: usize = 2;
  let pickup_similar_word_num: usize = 5;
  let word_dic = get_dictionary();

  let mut output = TypoChecResult::new();
  let mut similar_word_list: Vec<SimilarWord> = Vec::new();

  let mut select_word_upper_index: usize = 10;
  let mut select_word_lower_index = 0;

  
  if check_word_length==1 {
      return output;
  }else if check_word_length==2 {
      select_word_upper_index = (check_word_length -2) + select_word_range;
      select_word_lower_index = check_word_length -2;
  }else if check_word_length==21 {
      select_word_upper_index = check_word_length -2;
      select_word_lower_index = (check_word_length -2) - select_word_range;
  } else {
      select_word_upper_index = (check_word_length -2) + select_word_range;
      select_word_lower_index = (check_word_length -2) - select_word_range;
  }

  let same_length_word_dic = &word_dic[check_word_length-2];
  let selected_lower_word_dic = &word_dic[select_word_lower_index..check_word_length-2];
  let selected_upper_word_dic = &word_dic[check_word_length-1..select_word_upper_index];

  // 完全に一致する単語を探索する
  for temp_word in same_length_word_dic.iter() {
    match temp_word {
        Some(word) => {
          let levenshtein_length = levenshtein(&check_word, &word);

          if levenshtein_length==0 {
              output.match_word = Some(word.to_string());
              output.similar_word_list = None;
              return output;
          } else {
            similar_word_list.push(SimilarWord::new(word.to_string(), levenshtein_length));
          }
        },
        None => break,
    };
  }

  // 類似する単語を探す(探す単語よりも文字数がselect_word_range少ないもの)
  similar_word_list = calculate_word_list_levenshtein_length(selected_lower_word_dic, &check_word, similar_word_list);

  // 類似する単語を探す(探す単語よりも文字数がselect_word_range多いもの)
  similar_word_list = calculate_word_list_levenshtein_length(selected_upper_word_dic, &check_word, similar_word_list);

  output.similar_word_list = Some(get_top_similar_words(similar_word_list, pickup_similar_word_num));

  output
}

#[cfg(test)]
mod tests {
}
