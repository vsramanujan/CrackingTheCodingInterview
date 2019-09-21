use std::collections::HashMap;

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

#[cfg(tests)]
mod test {

  use super::*;

  #[test]
  fn single_character() {
    let all_chars = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in all_chars {
      assert_eq!(is_unique(i), true);
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
       "breakdowns"
       ];

    for i in words {
      assert_eq!(is_unique(i), true);
    }
  }
}
