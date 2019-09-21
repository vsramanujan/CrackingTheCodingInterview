use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
  let mut hash1 = HashMap::new();
  let mut hash2 = HashMap::new();

  let us1 = s1.to_lowercase();
  let us2 = s2.to_lowercase();

  for c in us1.chars() {
    *hash1.entry(c).or_insert(0) += 1;
  }

  for c in us2.chars() {
    *hash2.entry(c).or_insert(0) += 1;
  }

  return hash1 == hash2;
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn anagram_words() {
    let input = ["Arc", "Elbow", "State", "Cider", "Stressed"];
    let output = ["Car", "Below", "Taste", "Cried", "Desserts"];

    for (i, j) in input.iter().zip(output.iter()) {
      assert_eq!(is_permutation(&i, &j), true);
    }
  }

  #[test]
  fn anagram_words_with_an_extra_present_character() {
    let input = ["Arc", "Elbow", "State", "Cider", "Stressed"];
    let output = ["Carr", "Belowe", "Tastes", "Criedc", "Dessertsr"];

    for (i, j) in input.iter().zip(output.iter()) {
      assert_eq!(is_permutation(&i, &j), false);
    }
  }

  #[test]
  fn empty() {
    assert_eq!(is_permutation("", ""), true);
  }

}
