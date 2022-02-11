use crate::*;
use std::iter::Iterator;

pub struct JSONSerializer<CONTEXT> {
    s: String,
    indentation: u16,
    added_comma: bool,
    context: CONTEXT,
}

impl JSONSerializer<()> {
    pub fn new() -> Self {
        Self::new_with_context(())
    }
}

impl Default for JSONSerializer<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CONTEXT> JSONSerializer<CONTEXT> {
    fn new_with_context(context: CONTEXT) -> Self {
        JSONSerializer {
            s: String::new(),
            indentation: 0,
            added_comma: false,
            context,
        }
    }

    fn indent(&mut self) {
        self.s.extend((0..self.indentation).map(|_| ' '))
    }
}

impl<CONTEXT> Serializer for JSONSerializer<CONTEXT> {
    type Context = CONTEXT;
    type Result = String;

    fn f64(&mut self, n: f64) {
        self.s.push_str(&n.to_string())
    }

    fn i64(&mut self, n: i64) {
        self.s.push_str(&n.to_string())
    }

    fn bool(&mut self, b: bool) {
        if b {
            self.s.push_str("true")
        } else {
            self.s.push_str("false")
        }
    }

    fn string(&mut self, s: &str) {
        self.s.push('\"');
        self.s.push_str(s);
        self.s.push('\"');
    }

    fn null(&mut self) {
        self.s.push_str("null");
    }

    fn done(self) -> Self::Result {
        self.s
    }

    fn begin_array(&mut self) {
        self.added_comma = false;
        self.s.push('[');
    }

    fn begin_object(&mut self) {
        self.added_comma = false;
        self.s.push('{');
        self.indentation += 4;
    }

    fn property(&mut self, name: &str) {
        self.s.push('\n');
        self.indent();
        name.serialize(self);
        self.s.push_str(": ");
    }

    fn end_object(&mut self) {
        self.indentation -= 4;

        if self.added_comma {
            self.s.pop();
            self.s.pop();
            self.added_comma = false;
        }
        self.s.push('\n');
        self.indent();
        self.s.push('}');
    }

    fn value<V: Serialize<Self>>(&mut self, value: &V) {
        value.serialize(self);
        self.s += ", ";
        self.added_comma = true;
    }

    fn end_array(&mut self) {
        if self.added_comma {
            self.s.pop();
            self.s.pop();
            self.added_comma = false;
        }
        self.s.push(']');
    }

    fn get_context(&self) -> &Self::Context {
        &self.context
    }

    fn get_context_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }
}

pub trait ToJson: Sized {
    fn to_json(&self) -> String;
}
impl<T: Serialize<JSONSerializer<()>>> ToJson for T {
    fn to_json(&self) -> String {
        let mut serializer = JSONSerializer::new();
        self.serialize(&mut serializer);
        serializer.done()
    }
}
