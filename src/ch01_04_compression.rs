pub fn compress(string: &str) -> String {
    let mut compress_string = String::new();
    let mut last_char: char = '-';
    let mut current_run: i32 = 0;

    for (i, c) in string.char_indices() {
        if i == 0 {
            last_char = c;
            current_run = 1;
        } else if last_char == c {
            current_run += 1;
        } else {
          append_char_with_frequency(last_char,current_run,&mut compress_string);
            last_char = c;
            current_run = 1;
        }
    }

    // Flush out whatever you have on string end
    append_char_with_frequency(last_char, current_run, &mut compress_string);

    if compress_string.len() < string.len() {
      compress_string
    } else {
      string.to_string()
    }
}

fn append_char_with_frequency(c: char, freq: i32, string: &mut String) {
  let mut string_to_be_appended = c.to_string();
  string_to_be_appended.push_str(freq.to_string().as_str());
  string.push_str(string_to_be_appended.as_str());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn runs() {
        assert_eq!(compress("aaabbbcccddd"), "a3b3c3d3");
        assert_eq!(compress("abcd"), "abcd");
        assert_eq!(compress("a"), "a");
    }
}
