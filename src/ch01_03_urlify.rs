// Normal implementation
pub fn urlify(string: &str) -> String {
    let mut changed_string = String::new();

    for i in string.chars() {
        if i.to_string() == " " {
            changed_string.push_str("%20");
        } else {
            changed_string.push(i);
        }
    }

    changed_string
}

// Adv implementation - flex flex flex
#[allow(dead_code)]
pub fn urlify_flex(string: &str) -> String {
    string.trim().replace(" ", "%20")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn no_space() {
        assert_eq!("www.facebook.com", urlify("www.facebook.com"));
    }

    #[test]
    fn with_space() {
        assert_eq!(
            "www.facebook.com/well%20that%20sucks",
            urlify("www.facebook.com/well that sucks")
        );
    }
}
