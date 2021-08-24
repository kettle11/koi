use crate::*;
use std::iter::Iterator;

pub struct JSONSerializer<CONTEXT> {
    s: String,
    indentation: u16,
    just_began_object_or_array: bool,
    context: CONTEXT,
}

impl JSONSerializer<()> {
    pub fn new() -> Self {
        Self::new_with_context(())
    }
}

impl<CONTEXT> JSONSerializer<CONTEXT> {
    fn new_with_context(context: CONTEXT) -> Self {
        JSONSerializer {
            s: String::new(),
            indentation: 0,
            just_began_object_or_array: false,
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
        self.s.push_str("[");
        self.just_began_object_or_array = true;
    }

    fn begin_object(&mut self) {
        self.s.push('{');
        self.indentation += 4;
        self.just_began_object_or_array = true;
    }

    fn property<V: Serialize<Self>>(&mut self, name: &str, value: &V) {
        if !self.just_began_object_or_array {
            self.s.push(',');
        }
        self.just_began_object_or_array = false;
        self.s.push('\n');
        self.indent();
        name.serialize(self);
        self.s.push_str(": ");
        value.serialize(self);
    }

    fn end_object(&mut self) {
        self.indentation -= 4;
        self.s.push('\n');
        self.indent();
        self.s.push('}');
    }

    fn value<V: Serialize<Self>>(&mut self, value: &V) {
        if !self.just_began_object_or_array {
            self.s += ", "
        }
        self.just_began_object_or_array = false;
        value.serialize(self);
    }

    fn end_array(&mut self) {
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
