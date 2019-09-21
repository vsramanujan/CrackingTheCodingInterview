use std::collections::HashMap;
use std::collections::HashSet;

// Using HashMaps - the obvious way
#[allow(dead_code)]
pub fn is_unique(string: &str) -> bool {
  let mut char_hash = HashMap::new();

  for i in string.chars() {
    let is_absent = match char_hash.insert(i, 1) {
      Some(_val) => false,
      None => true,
    };

    if is_absent == false {
      return false;
    }
  }
  true
}

// Using HashSet - flex flex flex!

pub fn is_unique_flex(string: &str) -> bool {
  let mut hash = HashSet::new();

  !string.chars().any(|c| !hash.insert(c))
}

// Mutating input string
pub fn is_unique_mutate(string: &mut String) -> bool {
  let mut vec_chars: Vec<char> = string.chars().collect();
  vec_chars.sort();

  let mut vec_chars_for_dedup: Vec<char> = string.chars().collect();
  vec_chars_for_dedup.sort();
  vec_chars_for_dedup.dedup();

  vec_chars == vec_chars_for_dedup
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn single_character() {
    let all_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in all_chars {
      assert_eq!(is_unique(&i.to_string()[..]), true);
      assert_eq!(is_unique_flex(&i.to_string()[..]), true);
      assert_eq!(is_unique_mutate(&mut i.to_string()), true);
    }
  }

  #[test]
  fn unique_words() {
    let words = [
      "abductions",
      "abridgment",
      "admixtures",
      "afterglows",
      "aftershock",
      "algorithms",
      "amplitudes",
      "anchorites",
      "angiosperm",
      "angleworms",
      "artichokes",
      "atrophying",
      "authorized",
      "authorizes",
      "autopsying",
      "backfields",
      "background",
      "backslider",
      "bandoliers",
      "bankruptcy",
      "bankrupted",
      "becomingly",
      "benchmarks",
      "bifurcated",
      "bifurcates",
      "binoculars",
      "birthplace",
      "bivouacked",
      "blacksmith",
      "blackthorn",
      "blockading",
      "blockheads",
      "blueprints",
      "blustering",
      "bolstering",
      "boulevards",
      "boundaries",
      "boyfriends",
      "bracketing",
      "breakdowns",
    ];

    for i in words.iter() {
      assert_eq!(is_unique(i), true);
      assert_eq!(is_unique_flex(i), true);
      assert_eq!(is_unique_mutate(&mut i.to_string()), true);
    }
  }

  #[test]
  fn incorect_words() {
    let words = ["aaa", "bbb", "abab", "zxvbmaoz", "??"];
    for i in words.iter() {
      assert_eq!(is_unique(i), false);
      assert_eq!(is_unique_flex(i), false);
      assert_eq!(is_unique_mutate(&mut i.to_string()), false);
    }
  }
}
