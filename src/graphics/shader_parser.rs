use std::collections::HashMap;

struct ShaderParser<'a> {
    source: &'a str,
    iter: std::iter::Peekable<std::str::CharIndices<'a>>,
    position: usize,
}

impl<'a> ShaderParser<'a> {
    fn skip_whitespace(&mut self) {
        loop {
            match self.iter.peek().cloned() {
                Some((i, c)) if c.is_whitespace() => {
                    self.iter.next();
                    self.position = i + 1;
                }
                _ => break,
            }
        }
    }

    fn read_word(&mut self) -> &'a str {
        self.skip_whitespace();
        let start = self.position;
        loop {
            match self.iter.peek().cloned() {
                Some((i, c)) if c.is_alphanumeric() || c == '_' => {
                    self.iter.next();
                    self.position = i + 1;
                }
                _ => break,
            }
        }

        &self.source[start..self.position]
    }

    fn read_command(&mut self) -> Option<&'a str> {
        self.skip_whitespace();

        // Check for the `#` character
        match self.iter.peek().cloned() {
            Some((i, '#')) => {
                self.iter.next();
                self.position = i + 1;
            }
            _ => return None,
        }

        Some(self.read_word())
    }

    fn read_stretch(&mut self) -> &'a str {
        let start = self.position;
        loop {
            match self.iter.peek().cloned() {
                Some((i, c)) if c != '#' => {
                    self.iter.next();
                    self.position = i + 1;
                }
                _ => break,
            }
        }

        &self.source[start..self.position]
    }

    fn parse(
        source: &'a str,
        snippets: &'a HashMap<&'static str, &'static str>,
        prepend: &'a str,
    ) -> (String, String) {
        let mut parser = Self {
            source,
            iter: source.char_indices().peekable(),
            position: 0,
        };

        let mut vertex = String::new();
        let mut fragment = String::with_capacity(source.len());
        let mut current_string = String::with_capacity(source.len());

        current_string += prepend;
        // Ignore anything before the first command
        let _ = parser.read_stretch();

        loop {
            match parser.read_command() {
                None => {
                    break;
                }
                Some("VERTEX") => {}
                Some("FRAGMENT") => {
                    std::mem::swap(&mut current_string, &mut vertex);
                    current_string += prepend;
                }
                Some("INSERT" | "INCLUDE") => {
                    let key = parser.read_word();
                    if !key.is_empty() {
                        if let Some(snippet) = snippets.get(key) {
                            current_string += snippet;
                        } else {
                            crate::log!(
                                "SHADER ERROR: No shader snippet with a matching name: {:?}",
                                key
                            );
                        }
                    } else {
                        crate::log!("SHADER ERROR: Expected key after shader include");
                    }
                }
                Some("") => {
                    crate::log!("SHADER ERROR: Expected keyword after '#'");
                }
                Some(s) => {
                    crate::log!("SHADER ERROR: Unknown keyword: {:?}", s);
                }
            }

            let next_stretch = parser.read_stretch();
            current_string += next_stretch;
        }

        std::mem::swap(&mut fragment, &mut current_string);

        (vertex, fragment)
    }
}

pub fn parse_shader(
    snippets: &HashMap<&'static str, &'static str>,
    source: &str,
    prepend: &str,
) -> (String, String) {
    ShaderParser::parse(source, snippets, prepend)
}
