macro_rules! encode_characters {
    ($text:expr) => {{
        let mut result = String::with_capacity($text.len());
        let mut iterator = $text.chars().peekable();

        while let Some(character) = iterator.next() {
            if character != '\\' {
                result.push(character);
                continue;
            }

            match iterator.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('0') => result.push('\0'),
                Some('u') => {
                    if Some('{') == iterator.next() {
                        let mut hex = String::new();
                        while let Some(&next_character) = iterator.peek() {
                            if next_character == '}' {
                                iterator.next(); // consume
                                break;
                            }
                            hex.push(next_character);
                            iterator.next(); // consume
                        }
                        match u32::from_str_radix(&hex, 16) {
                            Ok(codepoint) => match char::from_u32(codepoint) {
                                Some(unicode_character) => result.push(unicode_character),
                                None => result.push_str(&format!("\\u{{{}}}", hex)), // TODO(Chloe)
                                                                                     // Warning
                                                                                     // here
                            },
                            Err(_) => result.push_str(&format!("\\u{{{}}}", hex)), // TODO(Chloe)
                                                                                   // Warning here
                        }
                    } else {
                        result.push_str("\\u");
                    }
                }
                Some(peek) => {
                    result.push('\\');
                    result.push(peek);
                    // Warning here for unknown escape
                }
                None => result.push('\\'),
            }
        }
        result
    }};
}

macro_rules! make_keywords {
    () => {};

    ($($arg:ident), *) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum KeywordKind {
            $($arg),*    
        }
        
        impl KeywordKind {
            pub fn create_keyword(identifier: &str) -> Option<Self> {
                match identifier.to_ascii_lowercase().as_str() {
                    $(
                        hack if hack == stringify!($arg).to_ascii_lowercase() => Some(KeywordKind::$arg), // HACK!
                    )*
                    _ => None,
                }
            }
        }
    }
}
