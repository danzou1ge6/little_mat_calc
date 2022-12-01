/// Determines whether a character is a *Delimitator*
///
/// For example, a `(` splits contents before it, after it and append itself
/// to the token stream
fn is_delimitator(chr: char) -> bool {
    match chr {
        '(' => true,
        ')' => true,
        '[' => true,
        ']' => true,
        ';' => true,
        _ => false,
    }
}

/// Determine whether a character is a *Whitespace*
///
/// *Whitespaces* are discarded, and also serve as a seperator
fn is_whitespace(chr: char) -> bool {
    match chr {
        ' ' => true,
        '\t' => true,
        '\n' => true,
        _ => false,
    }
}

/// Read from a [`&str`] and iter over its *Pieces*, which can be *Tokenized*
pub struct SplitBuffer<'a> {
    left: &'a str,
}

impl<'a> SplitBuffer<'a> {
    /// The [`String`] it is constructed from should live longer than it does.
    pub fn new(s: &'a str) -> Self {
        SplitBuffer { left: s }
    }
}

impl<'a> Iterator for SplitBuffer<'a> {
    type Item = &'a str;

    /// Iters through a [`&str`] and yields *Pieces*
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.left.chars(); // Creation of `Chars` costs minimium since it's lazy

        loop {
            if let Some(chr) = chars.next() {
                if is_whitespace(chr) {
                    self.left = &self.left[chr.len_utf8()..];
                } else {
                    break;
                }
            } else {
                return None;
            }
        }

        let mut chars = self.left.chars();

        if let Some(chr) = chars.next() {
            if is_delimitator(chr) {
                let ret = &self.left[0..chr.len_utf8()];
                self.left = &self.left[chr.len_utf8()..];
                return Some(ret);
            }
        } else {
            return None;
        }

        let mut chars = self.left.chars();
        let mut end = 0;

        loop {
            if let Some(chr) = chars.next() {
                if is_whitespace(chr) || is_delimitator(chr) {
                    break;
                }
                end += chr.len_utf8();
            } else {
                break;
            }
        }

        let ret = &self.left[..end];
        self.left = &self.left[end..];
        return Some(ret);
    }
}

#[cfg(test)]
mod test {
    use super::SplitBuffer;
    #[test]
    fn test() {
        let s = String::from("[1 1; 2 3;]");
        let sb = SplitBuffer::new(&s);

        let expected = vec!["[", "1", "1", ";", "2", "3", ";", "]"];
        let result: Vec<&str> = sb.collect();

        assert_eq!(expected, result);
    }
}