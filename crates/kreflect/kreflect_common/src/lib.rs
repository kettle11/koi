use std::{borrow::Cow, usize};

#[derive(Debug, Clone)]
pub enum Token<'a> {
    For,
    Struct,
    Enum,
    Fn,
    In,
    Pub,
    Use,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    Colon,
    Plus,
    Star,
    // ::
    Colon2,
    Comma,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    OpenParentheses,
    CloseParentheses,
    Pound,
    Crate,
    SelfLowercase,
    Super,
    SingleQuote,
    SemiColon,
    Dyn,
    And,
    Mut,
    Not,
    Equal,
    Const,
    CharLiteral(char),
    StringLiteral(String),
    LiteralBool(bool),
    IntegerLiteral(i128),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Identifier(Cow<'a, str>),
    RArrow,
}

// Convert TokenStream into a more generic format.
/// This may seem a bit weird, but this could allow for partial standalone Rust
/// parsing in a future version.
#[cfg(feature = "proc_macro")]
extern crate proc_macro;

fn char_to_token<'a>(c: char) -> Token<'a> {
    match c {
        ':' => Token::Colon,
        ',' => Token::Comma,
        '<' => Token::LessThan,
        '>' => Token::GreaterThan,
        '#' => Token::Pound,
        '\'' => Token::SingleQuote,
        ';' => Token::SemiColon,
        '&' => Token::And,
        '!' => Token::Not,
        '=' => Token::Equal,
        '+' => Token::Plus,
        '*' => Token::Star,
        _ => unimplemented!("Unimplemented punctuation: {}", c),
    }
}
#[cfg(feature = "proc_macro")]
pub fn token_stream_to_rust_tokens(
    token_stream: proc_macro::TokenStream,
    rust_tokens: &mut Vec<Token>,
) {
    use proc_macro::{Delimiter, Spacing, TokenTree};
    use std::str::FromStr;

    let mut token_iter = token_stream.into_iter();
    while let Some(token) = token_iter.next() {
        match token {
            TokenTree::Ident(i) => {
                let i = i.to_string();
                let token = match i.as_str() {
                    "struct" => Token::Struct,
                    "enum" => Token::Enum,
                    "fn" => Token::Fn,
                    "for" => Token::For,
                    "in" => Token::In,
                    "pub" => Token::Pub,
                    "use" => Token::Use,
                    "crate" => Token::Crate,
                    "self" => Token::SelfLowercase,
                    "super" => Token::Super,
                    "dyn" => Token::Dyn,
                    "mut" => Token::Mut,
                    "const" => Token::Const,
                    _ => Token::Identifier(Cow::Owned(i)),
                };
                rust_tokens.push(token);
            }
            TokenTree::Punct(p) => {
                let c0 = p.as_char();
                let token = match p.spacing() {
                    Spacing::Alone => char_to_token(c0),
                    Spacing::Joint => {
                        let next_c = token_iter.next();
                        match next_c {
                            Some(TokenTree::Punct(p)) => {
                                let c1 = p.as_char();
                                match (c0, c1) {
                                    ('+', '=') => Token::PlusEqual,
                                    ('-', '=') => Token::MinusEqual,
                                    ('*', '=') => Token::TimesEqual,
                                    ('<', '=') => Token::LessThanOrEqual,
                                    ('>', '=') => Token::GreaterThanOrEqual,
                                    (':', ':') => Token::Colon2,
                                    ('-', '>') => Token::RArrow,
                                    (a, b) => {
                                        rust_tokens.push(char_to_token(a));
                                        char_to_token(b)
                                    }
                                }
                            }
                            // This happens for lifetimes.
                            Some(TokenTree::Ident(i)) => match c0 {
                                '\'' => {
                                    rust_tokens.push(Token::SingleQuote);
                                    Token::Identifier(Cow::Owned(i.to_string()))
                                }
                                _ => unimplemented!("Unexpected case"),
                            },
                            // I don't know why this case happens but it happens only within
                            // Rust-analyzer for some reason.
                            None => char_to_token(c0),
                            _ => {
                                unimplemented!("UNEXPECTED: C0: {:?}, {:?}", c0, next_c)
                            }
                        }
                    }
                };
                rust_tokens.push(token);
            }
            TokenTree::Group(g) => {
                let (open, close) = match g.delimiter() {
                    Delimiter::Brace => (Some(Token::OpenBrace), Some(Token::CloseBrace)),
                    Delimiter::Bracket => (Some(Token::OpenBracket), Some(Token::CloseBracket)),
                    Delimiter::Parenthesis => {
                        (Some(Token::OpenParentheses), Some(Token::CloseParentheses))
                    }
                    Delimiter::None => (None, None),
                };
                if let Some(t) = open {
                    rust_tokens.push(t)
                }
                token_stream_to_rust_tokens(g.stream(), rust_tokens);
                if let Some(t) = close {
                    rust_tokens.push(t)
                }
            }
            TokenTree::Literal(l) => {
                let s = l.to_string();
                let literal = match s.as_str() {
                    "true" => Token::BoolLiteral(true),
                    "false" => Token::BoolLiteral(false),
                    _ => {
                        if let Ok(i) = i128::from_str(&s) {
                            Token::IntegerLiteral(i)
                        } else if let Ok(f) = f64::from_str(&s) {
                            Token::FloatLiteral(f)
                        } else if let Ok(c) = char::from_str(&s) {
                            Token::CharLiteral(c)
                        } else if let Ok(s) = String::from_str(&s) {
                            Token::StringLiteral(s)
                        } else {
                            panic!("Unhandled literal type");
                        }
                    }
                };
                rust_tokens.push(literal);
            }
        }
    }
}

#[derive(Debug)]
pub enum Visibility {
    Private,
    Pub,
    Crate,
    Super,
}

impl Visibility {
    pub fn as_string(&self) -> String {
        match self {
            Visibility::Private => "",
            Visibility::Pub => "pub",
            Visibility::Crate => "crate",
            Visibility::Super => "super",
        }
        .to_string()
    }
}
#[derive(Debug)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl Mutability {
    pub fn as_string(&self) -> String {
        match self {
            Mutability::Mutable => "mut",
            Mutability::Immutable => "",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub enum TypeParamBound<'a> {
    Trait(Path<'a>),
    Lifetime(Cow<'a, str>),
}

#[derive(Debug)]
pub struct TypeParamBounds<'a>(Vec<TypeParamBound<'a>>);

impl<'a> TypeParamBounds<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        let mut first = true;
        for bound in &self.0 {
            if !first {
                string += " + ";
            }
            first = false;
            match &bound {
                TypeParamBound::Lifetime(i) => {
                    string.push('\'');
                    string += i;
                }
                TypeParamBound::Trait(path) => string += &path.as_string(),
            }
        }
        string
    }
}

#[derive(Debug)]
pub enum Type<'a> {
    Name(Path<'a>),
    Tuple(Vec<Type<'a>>),
    Array {
        _type: Box<Type<'a>>,
        size: Expression<'a>,
    },
    Reference {
        lifetime: Cow<'a, str>,
        mutability: Mutability,
        _type: Box<Type<'a>>,
    },
    TraitObject(TypeParamBounds<'a>),
    RawPointer {
        mutability: Mutability,
        _type: Box<Type<'a>>,
    },
    FunctionPointer {
        parameters: Vec<Type<'a>>,
        _return: Option<Box<Type<'a>>>,
    },
}

impl<'a> Type<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        match &self {
            Type::Name(name) => {
                string += &name.as_string();
            }
            Type::Tuple(types) => {
                string.push('(');
                for _type in types {
                    string += &_type.as_string();
                    string.push(',');
                }
                string.push(')');
            }
            Type::FunctionPointer {
                parameters,
                _return,
            } => {
                string += "fn (";
                for _type in parameters {
                    string += &_type.as_string();
                    string.push(',');
                }
                string.push(')');
                if let Some(_return) = _return {
                    string += "-> ";
                    string += &_return.as_string();
                }
            }
            Type::Array { _type, size } => {
                string.push('[');
                string += &_type.as_string();
                string.push(';');
                string += &size.as_string();
                string.push(']');
            }
            Type::RawPointer { mutability, _type } => {
                string.push('*');
                string.push('\'');
                match mutability {
                    Mutability::Immutable => string += " const ",
                    Mutability::Mutable => string += " mut ",
                }
                string += &_type.as_string()
            }
            Type::Reference {
                lifetime,
                mutability,
                _type,
            } => {
                string.push('&');
                string.push('\'');
                string += lifetime;
                match mutability {
                    Mutability::Immutable => {}
                    Mutability::Mutable => string += " mut ",
                }
                string += &_type.as_string()
            }
            Type::TraitObject(trait_bounds) => {
                string += "dyn ";
                string += &trait_bounds.as_string();
            }
        }
        string
    }
}

#[derive(Debug)]
pub enum AttributeStyle {
    Outer,
    Inner,
}

#[derive(Debug)]
pub struct Attribute<'a> {
    pub path: Path<'a>,
    pub tokens: Vec<Token<'a>>,
    pub attribute_style: AttributeStyle,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: Option<Cow<'a, str>>,
    pub _type: Type<'a>,
    pub visibility: Visibility,
}

#[derive(Debug)]
pub enum Fields<'a> {
    Tuple(Vec<Field<'a>>),
    Struct(Vec<Field<'a>>),
    Unit,
}

impl<'a> Fields<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        match &self {
            Self::Tuple(_) => {
                todo!("Kreflect does not support tuple fields yet")
            }
            Self::Struct(fields) => {
                string += "{\n";
                for field in fields {
                    string += "    ";
                    string += &field.visibility.as_string();
                    string += field.name.as_ref().unwrap();
                    string.push(':');
                    string += &field._type.as_string();
                    string += ",\n";
                }
                string.push('}');
            }
            Self::Unit => {
                string.push(';');
            }
        }
        string
    }
}

// A `Struct` declaration
#[derive(Debug)]
pub struct Struct<'a> {
    pub name: Cow<'a, str>,
    pub visibility: Visibility,
    pub fields: Fields<'a>,
    pub generic_parameters: GenericParams<'a>,
}

impl<'a> Struct<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        string += &self.visibility.as_string();
        string += "struct";
        string += &self.name;
        let (lifetimes, types, consts) = self.generic_parameters.lifetimes_types_consts();
        string += &lifetimes;
        string += &types;
        string += &consts;
        string += &self.fields.as_string();
        string
    }
}

#[derive(Debug)]
pub struct EnumVariant<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: Cow<'a, str>,
    pub fields: Fields<'a>,
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub name: Cow<'a, str>,
    pub visibility: Visibility,
    pub variants: Vec<EnumVariant<'a>>,
    pub generic_parameters: GenericParams<'a>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self { tokens, i: 0 }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(self.i)
    }

    fn next(&mut self) -> Option<&'a Token<'a>> {
        self.i += 1;
        self.tokens.get(self.i - 1)
    }

    fn advance(&mut self) {
        self.i += 1;
    }

    fn check_for_token(&mut self, expect: Token<'a>) -> Option<&'a Token<'a>> {
        let t = self.tokens.get(self.i)?;
        if std::mem::discriminant(t) == std::mem::discriminant(&expect) {
            self.i += 1;
            Some(t)
        } else {
            None
        }
    }

    fn check_for_identifier(&mut self) -> Option<Cow<'a, str>> {
        let t = self.tokens.get(self.i)?;
        if let Token::Identifier(t) = t {
            self.i += 1;
            Some(t.clone())
        } else {
            None
        }
    }

    fn visibility(&mut self) -> Option<Visibility> {
        Some(match self.peek()? {
            Token::Pub => {
                self.advance();
                match self.peek()? {
                    Token::OpenParentheses => {
                        self.advance();
                        let visibility = match self.next()? {
                            Token::Crate => Visibility::Crate,
                            Token::Super => Visibility::Super,
                            Token::SelfLowercase => Visibility::Private,
                            Token::In => {
                                panic!("'pub (in ... )' is not yet supported");
                            }
                            _ => {
                                self.i -= 1;
                                None?
                            }
                        };
                        self.check_for_token(Token::CloseParentheses)?;
                        visibility
                    }
                    _ => Visibility::Pub,
                }
            }
            _ => Visibility::Private,
        })
    }

    fn path(&mut self) -> Option<Path<'a>> {
        let mut segments = Vec::new();
        loop {
            let mut args = Vec::new();

            let path_segment_type = match self.peek() {
                Some(Token::Identifier(i)) => {
                    self.advance();
                    if self.check_for_token(Token::LessThan).is_some() {
                        loop {
                            match self.peek()? {
                                Token::GreaterThan => {
                                    self.advance();
                                    break;
                                }
                                Token::SingleQuote => {
                                    self.advance();
                                    let identifier = self.check_for_identifier()?;
                                    args.push(GenericArgument::Lifetime(identifier))
                                }
                                _ => {
                                    let _type = self._type();
                                    match _type {
                                        Some(t) => {
                                            args.push(GenericArgument::Type(t));
                                        }
                                        None => {
                                            let expression = self.expression()?;
                                            args.push(GenericArgument::Expression(expression));
                                        }
                                    }
                                }
                            }
                            self.check_for_token(Token::Comma);
                        }
                    }
                    PathSegmentType::Named(i.clone())
                }
                Some(Token::Crate) => {
                    self.advance();
                    PathSegmentType::Crate
                }
                Some(Token::Super) => {
                    self.advance();
                    PathSegmentType::Super
                }
                Some(Token::SelfLowercase) => {
                    self.advance();
                    PathSegmentType::SelfLowercase
                }
                _ => {
                    break;
                }
            };

            segments.push(PathSegment {
                path_segment_type,
                args,
            });
            if self.check_for_token(Token::Colon2).is_none() {
                break;
            }
        }
        Some(Path { segments })
    }

    // Extremely limited expression parsing.
    fn expression(&mut self) -> Option<Expression<'a>> {
        Some(match self.peek()? {
            Token::LiteralBool(b) => {
                self.advance();
                Expression::Literal(Literal::Bool(*b))
            }
            Token::FloatLiteral(b) => {
                self.advance();
                Expression::Literal(Literal::Float(*b))
            }
            Token::IntegerLiteral(b) => {
                self.advance();
                Expression::Literal(Literal::Integer(*b as i64))
            }
            Token::CharLiteral(b) => {
                self.advance();
                Expression::Literal(Literal::Char(*b))
            }
            Token::StringLiteral(b) => {
                self.advance();
                Expression::Literal(Literal::String(b.into()))
            }
            Token::Identifier(_) => {
                self.advance();
                Expression::Path(self.path()?)
            }
            _ => todo!("Unhandled expression"),
        })
    }

    fn type_param_bounds(&mut self) -> Option<TypeParamBounds<'a>> {
        let mut trait_bounds = Vec::new();
        loop {
            trait_bounds.push(match self.peek()? {
                Token::Identifier(_) => {
                    let path = self.path()?;
                    TypeParamBound::Trait(path)
                }
                Token::SingleQuote => {
                    self.advance();
                    let identifier = self.check_for_identifier()?;
                    TypeParamBound::Lifetime(identifier.clone())
                }
                _ => None?,
            });

            if self.check_for_token(Token::Plus).is_none() {
                break;
            }
        }
        Some(TypeParamBounds(trait_bounds))
    }

    fn _type(&mut self) -> Option<Type<'a>> {
        // Not fully implemented
        Some(match self.peek()? {
            Token::Identifier(_) | Token::Crate | Token::Super | Token::SelfLowercase => {
                Type::Name(self.path()?)
            }
            Token::OpenParentheses => {
                self.advance();
                let mut types = Vec::new();
                loop {
                    if let Some(Token::CloseParentheses) = self.peek() {
                        self.advance();
                        break;
                    }

                    let _type = self._type()?;
                    types.push(_type);
                    self.check_for_token(Token::Comma);
                }
                Type::Tuple(types)
            }
            Token::Dyn => {
                self.advance();
                Type::TraitObject(self.type_param_bounds()?)
            }
            Token::OpenBracket => {
                self.advance();
                let _type = self._type()?;
                self.check_for_token(Token::SemiColon)?;

                let size = self.expression().unwrap();
                self.check_for_token(Token::CloseBracket);
                Type::Array {
                    size,
                    _type: Box::new(_type),
                }
            }
            // Parse the reference case. `&'a mut`
            Token::And => {
                self.advance();
                self.check_for_token(Token::SingleQuote)?;
                let lifetime = self.check_for_identifier()?;
                let mutability = if self.check_for_token(Token::Mut).is_some() {
                    Mutability::Mutable
                } else {
                    Mutability::Immutable
                };
                let _type = self._type()?;
                Type::Reference {
                    lifetime,
                    mutability,
                    _type: Box::new(_type),
                }
            }
            Token::Star => {
                // A raw pointer

                self.advance();

                let mutability = if self.check_for_token(Token::Mut).is_some() {
                    Mutability::Mutable
                } else {
                    if self.check_for_token(Token::Const).is_some() {
                        Mutability::Immutable
                    } else {
                        None?
                    }
                };
                let _type = self._type()?;
                Type::RawPointer {
                    mutability,
                    _type: Box::new(_type),
                }
            }
            Token::Fn => {
                // A function pointer
                self.advance();
                self.check_for_token(Token::OpenParentheses)?;
                let mut parameters = Vec::new();
                loop {
                    if let Some(Token::CloseParentheses) = self.peek() {
                        self.advance();
                        break;
                    }

                    let _type = self._type()?;
                    parameters.push(_type);
                    self.check_for_token(Token::Comma);
                }
                let mut _return = None;
                if self.check_for_token(Token::RArrow).is_some() {
                    _return = self._type().map(Box::new);
                }
                Type::FunctionPointer {
                    parameters,
                    _return,
                }
            }
            _ => None?,
        })
    }

    fn attribute(&mut self) -> Option<Attribute<'a>> {
        self.check_for_token(Token::Pound)?;
        let attribute_style = if self.check_for_token(Token::Not).is_some() {
            AttributeStyle::Inner
        } else {
            AttributeStyle::Outer
        };
        self.check_for_token(Token::OpenBracket)?;
        let path = self.path()?;
        let mut tokens = Vec::new();
        while self.check_for_token(Token::CloseBracket).is_none() {
            tokens.push(self.next()?.clone())
        }
        Some(Attribute {
            path,
            tokens,
            attribute_style,
        })
    }

    fn enum_variants(&mut self) -> Option<Vec<EnumVariant<'a>>> {
        let mut variants = Vec::new();
        self.check_for_token(Token::OpenBrace)?;
        loop {
            let mut attributes = Vec::new();
            while let Some(attribute) = self.attribute() {
                attributes.push(attribute)
            }

            match self.next()? {
                Token::Identifier(name) => {
                    let fields = match self.peek()? {
                        Token::Comma | Token::CloseBrace => Fields::Unit,
                        Token::OpenBrace | Token::OpenParentheses => self.fields()?,
                        _ => return None,
                    };

                    variants.push(EnumVariant {
                        attributes,
                        fields,
                        name: name.clone(),
                    });
                    self.check_for_token(Token::Comma);
                }
                Token::CloseBrace => break,
                _ => return None,
            }
        }
        Some(variants)
    }

    fn fields(&mut self) -> Option<Fields<'a>> {
        let mut members = Vec::new();
        Some(match self.peek()? {
            Token::OpenBrace => {
                self.advance();
                loop {
                    let mut attributes = Vec::new();
                    while let Some(attribute) = self.attribute() {
                        attributes.push(attribute)
                    }
                    let visibility = self.visibility()?;

                    match self.next()? {
                        Token::Identifier(name) => {
                            self.check_for_token(Token::Colon)?;
                            let _type = self._type()?;

                            members.push(Field {
                                attributes,
                                name: Some(name.clone()),
                                _type,
                                visibility,
                            });
                            self.check_for_token(Token::Comma);
                        }
                        Token::CloseBrace => break,
                        _ => return None,
                    }
                }
                Fields::Struct(members)
            }
            Token::OpenParentheses => {
                self.advance();
                loop {
                    let mut attributes = Vec::new();
                    while let Some(attribute) = self.attribute() {
                        attributes.push(attribute)
                    }

                    let visibility = self.visibility()?;

                    match self.peek()? {
                        Token::CloseParentheses => {
                            self.advance();
                            break;
                        }
                        _ => {
                            let _type = self._type().expect("failed to parse type");
                            members.push(Field {
                                attributes,
                                name: None,
                                _type,
                                visibility,
                            });
                            self.check_for_token(Token::Comma);
                        }
                    }
                }
                Fields::Tuple(members)
            }
            v => {
                panic!("Unexpected token: {:?}", v)
            }
        })
    }

    /// Returns empty if there are no generic parameters.
    fn generic_params(&mut self) -> Option<GenericParams<'a>> {
        let mut generic_params = Vec::new();
        if let Some(Token::LessThan) = self.peek() {
            self.advance();
            loop {
                match self.peek() {
                    Some(Token::Identifier(identifier)) => {
                        self.advance();
                        generic_params.push(GenericParam::Type {
                            identifier: identifier.clone(),
                            type_bounds: if self.check_for_token(Token::Colon).is_some() {
                                self.type_param_bounds()?
                            } else {
                                TypeParamBounds(Vec::new())
                            },
                        })
                    }
                    Some(Token::SingleQuote) => {
                        self.advance();
                        let identifier = self
                            .check_for_identifier()
                            .expect("Expected identifier after '\''")
                            .clone();

                        // Should check for bounds here.
                        generic_params.push(GenericParam::Lifetime { identifier })
                    }
                    Some(Token::Const) => {
                        self.advance();
                        let identifier = self.check_for_identifier().unwrap();
                        self.check_for_token(Token::Colon).unwrap();
                        let _type = self._type().unwrap();
                        generic_params.push(GenericParam::Const { identifier, _type })
                    }
                    Some(Token::GreaterThan) => {
                        self.advance();
                        break;
                    }
                    other => panic!("Unexpected token in generic params: {:?}", other),
                }

                self.check_for_token(Token::Comma);
            }
        }

        Some(GenericParams(generic_params))
    }

    fn _struct(&mut self, visibility: Visibility) -> Option<Struct<'a>> {
        // The `struct` token was already parsed.
        self.advance();

        // Parse struct
        // Read name
        let name = self.check_for_identifier()?;

        // Check for generics
        let generic_parameters = self.generic_params()?;

        Some(match self.peek()? {
            Token::SemiColon => Struct {
                name,
                visibility,
                fields: Fields::Unit,
                generic_parameters,
            },
            _ => {
                let fields = self.fields().expect("Could not parse fields");
                Struct {
                    name,
                    visibility,
                    fields,
                    generic_parameters,
                }
            }
        })
    }

    fn _enum(&mut self, visibility: Visibility) -> Option<Enum<'a>> {
        // The `enum` token was already parsed.
        self.advance();

        let name = self.check_for_identifier()?;

        // Check for generics
        let generic_parameters = self.generic_params()?;
        let variants = self.enum_variants()?;

        Some(Enum {
            name,
            variants,
            visibility,
            generic_parameters,
        })
    }

    // Parses a top-level item declaration
    pub fn parse(&mut self) -> Option<Value<'a>> {
        // Skip other attributes for now.
        while self.attribute().is_some() {}
        let visibility = self.visibility().unwrap();
        Some(match self.peek()? {
            Token::Struct => Value::Struct(self._struct(visibility)?),
            Token::Enum => Value::Enum(self._enum(visibility)?),
            token => todo!(
                "Kreflect cannot parse non-struct non-enums yet: {:?}",
                token
            ),
        })
    }
}

#[derive(Debug)]
pub enum Literal<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Bool(bool),
    Char(char),
    Float(f64),
}

impl<'a> Literal<'a> {
    pub fn as_string(&self) -> String {
        match self {
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Integer(i) => i.to_string(),
            Literal::Bool(b) => b.to_string(),
            Literal::Char(c) => c.to_string(),
            Literal::Float(f) => f.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ConstParameter<'a> {
    pub identifier: Cow<'a, str>,
    pub _type: Type<'a>,
}

impl<'a> ConstParameter<'a> {
    pub fn as_string(&self) -> String {
        format!("const {}: {}", &self.identifier, self._type.as_string())
    }
}

#[derive(Debug)]
pub enum GenericParam<'a> {
    Type {
        identifier: Cow<'a, str>,
        type_bounds: TypeParamBounds<'a>,
    },
    Lifetime {
        identifier: Cow<'a, str>,
    },
    Const {
        identifier: Cow<'a, str>,
        _type: Type<'a>,
    },
}

impl<'a> GenericParam<'a> {
    pub fn as_string(&self) -> String {
        match self {
            Self::Type {
                identifier,
                type_bounds,
            } => {
                let type_bounds = type_bounds.as_string();
                if type_bounds.is_empty() {
                    identifier.to_string()
                } else {
                    format!("{}: {}", identifier, type_bounds)
                }
            }
            Self::Lifetime { identifier } => {
                format!("{}", identifier)
            }
            Self::Const { identifier, _type } => {
                format!("const {}: {}", identifier, _type.as_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct GenericParams<'a>(pub Vec<GenericParam<'a>>);

impl<'a> GenericParams<'a> {
    pub fn lifetimes_types_consts(&self) -> (String, String, String) {
        let mut lifetimes = String::new();
        let mut types = String::new();
        let mut consts = String::new();

        for generic_parameter in &self.0 {
            match generic_parameter {
                GenericParam::Lifetime { .. } => {
                    lifetimes += &generic_parameter.as_string();
                    lifetimes.push(',');
                }
                GenericParam::Type { .. } => {
                    types += &generic_parameter.as_string();
                    types.push(',');
                }
                GenericParam::Const { .. } => {
                    consts += &generic_parameter.as_string();
                    consts.push(',');
                }
            }
        }
        (lifetimes, types, consts)
    }

    // Gets these GenericParams formatted to be used as the part after `impl`
    pub fn as_impl_args(&self) -> String {
        let mut string = String::new();
        string.push('<');
        let (lifetimes, types, consts) = self.lifetimes_types_consts();
        string += &lifetimes;
        string += &types;
        string += &consts;
        string.push('>');
        string
    }

    // Gets these GenericParams formatted to be used as args.
    // `<T: Debug, const SIZE: usize>` would become `<T, SIZE>`
    pub fn as_args(&self) -> String {
        let mut string = String::new();
        if !self.0.is_empty() {
            string.push('<');
            for generic_parameter in &self.0 {
                match generic_parameter {
                    GenericParam::Lifetime { identifier, .. } => {
                        string.push('\'');
                        string += identifier
                    }
                    GenericParam::Type { identifier, .. } => string += identifier,
                    GenericParam::Const { identifier, .. } => string += identifier,
                }
                string.push(',');
            }
            string.push('>');
        }
        string
    }
}

#[derive(Debug)]
pub enum GenericArgument<'a> {
    Type(Type<'a>),
    Lifetime(Cow<'a, str>),
    Expression(Expression<'a>),
}

impl<'a> GenericArgument<'a> {
    pub fn as_string(&self) -> String {
        match self {
            GenericArgument::Type(t) => t.as_string(),
            GenericArgument::Lifetime(l) => format!("'{}", l),
            GenericArgument::Expression(e) => e.as_string(),
        }
    }
}

#[derive(Debug)]
pub enum PathSegmentType<'a> {
    Named(Cow<'a, str>),
    Crate,
    Super,
    SelfLowercase,
}
#[derive(Debug)]
pub struct PathSegment<'a> {
    pub path_segment_type: PathSegmentType<'a>,
    pub args: Vec<GenericArgument<'a>>,
}

#[derive(Debug)]
pub struct Path<'a> {
    pub segments: Vec<PathSegment<'a>>,
}

impl<'a> Path<'a> {
    pub fn new(segments: &[Cow<'a, str>]) -> Self {
        Self {
            segments: segments
                .iter()
                .map(|n| PathSegment {
                    path_segment_type: PathSegmentType::Named(n.clone()),
                    args: Vec::new(),
                })
                .collect(),
        }
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        let last = self.segments.len();
        for (i, segment) in self.segments.iter().enumerate() {
            string += match &segment.path_segment_type {
                PathSegmentType::Crate => "crate",
                PathSegmentType::SelfLowercase => "self",
                PathSegmentType::Super => "super",
                PathSegmentType::Named(name) => name,
            };
            if !segment.args.is_empty() {
                string += "<";
                for arg in &segment.args {
                    string += &arg.as_string();
                }
                string += ">";
            }
            if i != last - 1 {
                string += "::";
            }
        }
        string
    }
}

#[derive(Debug)]
pub struct ReferenceExpression<'a> {
    pub mutability: Mutability,
    pub expression: Box<Expression<'a>>,
}

impl<'a> ReferenceExpression<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        string += "& ";
        match self.mutability {
            Mutability::Mutable => string += "mut",
            Mutability::Immutable => {}
        }
        string
    }
}

#[derive(Debug)]
pub struct Macro<'a> {
    _path: Path<'a>,
    // tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
    Call {
        function: Box<Expression<'a>>,
        arguments: Vec<Expression<'a>>,
    },
    Macro(Macro<'a>),
    Path(Path<'a>),
    Reference(ReferenceExpression<'a>),
    Struct(StructExpression<'a>),
    // Just a string to be inserted
    Raw(String),
}

impl<'a> Expression<'a> {
    pub fn as_string(&self) -> String {
        match self {
            Expression::Literal(v) => v.as_string(),
            Expression::Call {
                function,
                arguments,
            } => {
                let mut string = function.as_string();
                string += "(";
                for argument in arguments {
                    string += &argument.as_string();
                    string += ",";
                }
                string += ")";
                string
            }
            Expression::Path(p) => p.as_string(),
            Expression::Reference(r) => r.as_string(),
            Expression::Struct(s) => s.as_string(),
            Expression::Raw(s) => s.clone(),
            Expression::Macro(_) => todo!("Macro as_string not implemented"),
        }
    }
}

#[derive(Debug)]
pub struct FieldValue<'a> {
    pub name: String,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct StructExpression<'a> {
    pub path: Path<'a>,
    pub fields: Vec<FieldValue<'a>>,
}

impl<'a> StructExpression<'a> {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        string += &self.path.as_string();
        string += " {\n";
        for field in &self.fields {
            string += "    ";
            string += &field.name;
            string += ": ";
            string += &field.expression.as_string();
            string += ",\n";
        }
        string += "}";
        string
    }
}

pub struct ItemStatic<'a> {
    pub name: Cow<'a, str>,
    pub mutability: Mutability,
    pub visibility: Visibility,
    pub _type: Type<'a>,
    pub expression: Expression<'a>,
}

impl<'a> ItemStatic<'a> {
    pub fn as_string(&self) -> String {
        let mut string = self.visibility.as_string();
        string += " static ";
        string += &self.mutability.as_string();
        string += &self.name;
        string += ": ";
        string += &self._type.as_string();
        string += " = ";
        string += &self.expression.as_string();
        string += ";";
        string
    }
}

pub enum Item<'a> {
    // A bunch of other things could be implemented here, but they
    // aren't needed for now.
    ItemStatic(ItemStatic<'a>),
    Struct(Struct<'a>),
}

impl<'a> Item<'a> {
    pub fn as_string(&self) -> String {
        match self {
            Item::ItemStatic(i) => i.as_string(),
            Item::Struct(s) => s.as_string(),
        }
    }
}

/*
#[test]
fn static_declaration() {
    let item_static = ItemStatic {
        name: "DATA".into(),
        mutability: Mutability::Immutable,
        visibility: Visibility::Private,
        _type: Type::Name(Path {
            segments: vec![PathSegment {
                name: "Value".into(),
                args: vec![GenericArgument::Lifetime("static".into())],
            }],
        }),
        expression: Expression::Call {
            function: Box::new(Expression::Path(Path::new(&[
                "kreflect".into(),
                "Value".into(),
                "Struct".into(),
            ]))),
            arguments: vec![Expression::Struct(StructExpression {
                path: Path::new(&["kreflect".into(), "Struct".into()]),
                fields: vec![FieldValue {
                    name: "name".into(),
                    expression: Expression::Literal(Literal::String("Thing".into())),
                }],
            })],
        },
    };

    let string = item_static.as_string();
    println!("{}", string);
}
*/
