use crate::{AnyValue, Deserialize, Deserializer};
use std::borrow::Cow;
use std::iter::Peekable;
use std::str::CharIndices;

const RECURSIVE_LIMIT: usize = 1024;

#[derive(Clone)]
pub struct JSONDeserializer<'a, CONTEXT> {
    recursive_depth: usize,
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
    context: CONTEXT,
}

impl<'a> JSONDeserializer<'a, ()> {
    pub fn new(source: &'a str) -> Self {
        Self {
            recursive_depth: 0,
            iter: source.char_indices().peekable(),
            source,
            context: (),
        }
    }
}

impl<'a, CONTEXT> Deserializer<'a> for JSONDeserializer<'a, CONTEXT> {
    type Context = CONTEXT;
    fn string(&mut self) -> Option<Cow<'a, str>> {
        self.skip_whitespace();
        self.parse_string()
    }

    fn bool(&mut self) -> Option<bool> {
        self.skip_whitespace();
        Some(match self.iter.next()?.1 {
            't' => {
                for _ in 0..4 {
                    self.iter.next()?;
                }
                true
            }
            'f' => {
                for _ in 0..5 {
                    self.iter.next()?;
                }
                false
            }
            _ => None?,
        })
    }

    fn i64(&mut self) -> Option<i64> {
        self.skip_whitespace();
        self.parse_number().map(|f| f as i64)
    }

    fn f64(&mut self) -> Option<f64> {
        self.skip_whitespace();
        self.parse_number()
    }

    fn any<'b>(&'b mut self) -> Option<AnyValue<'a>> {
        self.skip_whitespace();

        Some(match self.iter.peek()?.1 {
            '{' => {
                self.iter.next();
                if self.recursive_depth >= RECURSIVE_LIMIT {
                    return None;
                }
                self.recursive_depth += 1;
                AnyValue::Object
            }
            '[' => {
                self.iter.next();
                if self.recursive_depth >= RECURSIVE_LIMIT {
                    return None;
                }
                self.recursive_depth += 1;
                AnyValue::Array
            }
            '"' => AnyValue::String(self.parse_string()?),
            't' => {
                // Parse true
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                AnyValue::Bool(true)
            }
            'f' => {
                // Parse false
                // For now just assume all the characters are correct
                for _ in 0..5 {
                    self.iter.next()?;
                }
                AnyValue::Bool(false)
            }
            'n' => {
                // Parse null
                // For now just assume all the characters are correct
                for _ in 0..4 {
                    self.iter.next()?;
                }
                AnyValue::Null
            }
            '-' => AnyValue::Number(self.parse_number()?), // Parse negative number
            c if c.is_ascii_digit() => AnyValue::Number(self.parse_number()?),
            _ => return None,
        })
    }

    fn begin_object(&mut self) -> bool {
        if self.recursive_depth >= RECURSIVE_LIMIT {
            return false;
        }
        self.skip_whitespace();

        match self.iter.next() {
            Some((_, '{')) => {
                self.recursive_depth += 1;
                true
            }
            _ => false,
        }
    }

    fn has_property(&mut self) -> Option<Cow<'a, str>> {
        // '{' already parsed
        self.skip_whitespace();
        match self.iter.peek()? {
            (_, ',') => {
                self.iter.next();
            }
            (_, '}') => {
                if self.recursive_depth == 0 {
                    return None;
                } else {
                    self.recursive_depth -= 1
                };
                self.iter.next();
                None?
            }
            _ => {}
        }

        self.skip_whitespace();
        let name = self.parse_string()?;

        self.skip_whitespace();
        match self.iter.next() {
            Some((_, ':')) => {}
            _ => return None,
        };
        self.skip_whitespace();

        Some(name)
    }

    fn begin_array(&mut self) -> bool {
        if self.recursive_depth >= RECURSIVE_LIMIT {
            return false;
        }

        self.skip_whitespace();
        match self.iter.next() {
            Some((_, '[')) => {
                self.recursive_depth += 1;
                true
            }
            _ => false,
        }
    }

    fn has_array_value(&mut self) -> bool {
        // '[' already parsed
        self.skip_whitespace();
        match self.iter.peek() {
            Some((_, ',')) => {
                self.iter.next();
                true
            }
            Some((_, ']')) => {
                if self.recursive_depth == 0 {
                    return false;
                } else {
                    self.recursive_depth -= 1
                };
                self.iter.next();
                false
            }
            _ => true,
        }
    }

    fn get_context_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }
}

impl<'a, CONTEXT> JSONDeserializer<'a, CONTEXT> {
    pub fn skip_whitespace(&mut self) {
        while self.iter.peek().map_or(false, |(_, c)| c.is_whitespace()) {
            self.iter.next();
        }
    }

    pub fn parse_string(&mut self) -> Option<Cow<'a, str>> {
        let start_index = match self.iter.next() {
            Some((i, '"')) => i,
            _ => return None,
        };

        let mut string = Cow::from("");
        let mut owned = false;

        loop {
            match self.iter.next()? {
                (_, '"') => break,
                (_, '\\') => {
                    owned = true;
                    let next = self.iter.next()?;
                    match next.1 {
                        '\"' => string.to_mut().push('"'),
                        '/' => string.to_mut().push('/'),
                        '\\' => string.to_mut().push('\\'),
                        'n' => string.to_mut().push('\n'),
                        'b' => string.to_mut().push('\x08'),
                        'f' => string.to_mut().push('\x0C'),
                        'r' => string.to_mut().push('\r'),
                        't' => string.to_mut().push('\t'),
                        'u' => {
                            let slice = self.source.get(next.0 + 1..next.0 + 5)?;
                            let u = u32::from_str_radix(slice, 16).ok()?;
                            for _ in 0..4 {
                                self.iter.next();
                            }

                            let c = match u {
                                0xD800..=0xDBFF => {
                                    // This is a non-Basic Multilingual Plane (BMP) character
                                    // so it's encoded as two code points.

                                    // Skip the '\u'
                                    self.iter.next()?;
                                    let (start, _) = self.iter.next()?;

                                    let slice = self.source.get(start + 1..start + 5)?;
                                    let u1 = u32::from_str_radix(slice, 16).ok()?;
                                    if u1 < 0xDC00 || u1 > 0xDFFF {
                                        return None;
                                    }
                                    let n = (u32::from(u - 0xD800) << 10 | u32::from(u1 - 0xDC00))
                                        + 0x1_0000;

                                    for _ in 0..4 {
                                        self.iter.next();
                                    }
                                    std::char::from_u32(n)?
                                }
                                _ => std::char::from_u32(u)?,
                            };

                            string.to_mut().push(c);
                        }
                        _ => return None,
                    }
                }
                (i, c) => {
                    if owned {
                        string.to_mut().push(c)
                    } else {
                        string = Cow::from(&self.source[start_index + 1..i + c.len_utf8()])
                    }
                }
            }
        }
        Some(string)
    }

    pub fn parse_number(&mut self) -> Option<f64> {
        let is_negative = match self.iter.peek() {
            Some((_, '-')) => {
                self.iter.next();
                true
            }
            _ => false,
        };

        let mut number = 0.0;

        match self.iter.peek()?.1 {
            '0' => {
                self.iter.next();
            }
            c if c.is_ascii_digit() => loop {
                if let Some((_, c)) = self.iter.peek().cloned() {
                    if let Some(digit) = c.to_digit(10) {
                        number *= 10.;
                        number += digit as f64;
                        self.iter.next();
                        continue;
                    }
                }
                break;
            },
            _ => return None,
        }

        let mut position = 10.0;
        // Parse fraction
        match self.iter.peek() {
            Some((_, '.')) => {
                self.iter.next();
                // Parse fraction
                loop {
                    if let Some((_, c)) = self.iter.peek().cloned() {
                        if let Some(digit) = c.to_digit(10) {
                            number += digit as f64 / position;
                            position *= 10.0;
                            self.iter.next();
                            continue;
                        }
                    }
                    break;
                }
            }
            _ => {}
        }

        // Parse exponent
        match self.iter.peek() {
            Some((_, 'e')) | Some((_, 'E')) => {
                self.iter.next();
                let sign = match self.iter.peek()?.1 {
                    '-' => {
                        self.iter.next();
                        -1.
                    }
                    '+' => {
                        self.iter.next();
                        1.
                    }
                    _ => 1.,
                };

                // Skip leading zeroes
                loop {
                    match self.iter.peek() {
                        Some((_, '0')) => {
                            self.iter.next();
                        }
                        _ => break,
                    }
                }

                let mut exponent = 0.0;
                match self.iter.peek()?.1 {
                    '0' => {
                        self.iter.next();
                    }
                    c if c.is_ascii_digit() => loop {
                        if let Some((_, c)) = self.iter.peek().cloned() {
                            if let Some(digit) = c.to_digit(10) {
                                exponent *= 10.;
                                exponent += digit as f64;
                                self.iter.next();
                                continue;
                            }
                        }
                        break;
                    },
                    _ => return None,
                }

                number = number * (10.0f64).powf(exponent * sign);
            }
            _ => {}
        }

        if is_negative {
            number *= -1.0;
        }

        Some(number)
    }
}

pub trait FromJson<'a>: Sized {
    fn from_json(s: &'a str) -> Option<Self>;
}
impl<'a, T: Deserialize<'a, JSONDeserializer<'a, ()>>> FromJson<'a> for T {
    fn from_json(s: &'a str) -> Option<Self> {
        let mut deserializer = JSONDeserializer::new(s);
        Self::deserialize(&mut deserializer)
    }
}
