use std::str::Chars;
use std::cmp::min;

struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
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
pub fn generic_levenshtein<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> usize
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
pub fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}

pub fn normalized_levenshtein(a: &str, b: &str) -> f64 {
  if a.is_empty() && b.is_empty() {
      return 1.0;
  }
  1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64)
}

#[cfg(test)]
mod tests {
}
