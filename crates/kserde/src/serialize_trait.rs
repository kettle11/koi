use std::collections::HashMap;

pub trait Serialize<S: Serializer> {
    fn serialize(&self, serializer: &mut S);
}

pub trait Serializer: Sized {
    type Result;
    type Context;

    //fn new() -> Self;
    fn string(&mut self, s: &str);
    fn bool(&mut self, b: bool);
    fn i64(&mut self, i: i64);
    fn f64(&mut self, n: f64);
    fn null(&mut self);

    /// Serialize a value that implements Serialize.
    fn serialize<V: Serialize<Self>>(&mut self, value: &V) {
        V::serialize(value, self);
    }
    fn done(self) -> Self::Result;

    fn begin_object(&mut self);
    fn end_object(&mut self);
    /// Only call this in-between [begin_object] and [end_object] calls
    /// Assigns a property label to something but `value` must be separately called.
    fn property(&mut self, name: &str);

    fn begin_array(&mut self);
    fn end_array(&mut self);
    /// Only call this in-between [begin_array] and [end_array] calls
    fn value<V: Serialize<Self>>(&mut self, value: &V);

    fn get_context(&self) -> &Self::Context;
    fn get_context_mut(&mut self) -> &mut Self::Context;
}

impl<S: Serializer> Serialize<S> for &str {
    fn serialize(&self, serializer: &mut S) {
        serializer.string(self)
    }
}

impl<S: Serializer> Serialize<S> for String {
    fn serialize(&self, serializer: &mut S) {
        serializer.string(self)
    }
}

impl<S: Serializer> Serialize<S> for i32 {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.i64(*self as i64)
    }
}

impl<S: Serializer> Serialize<S> for i64 {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.i64(*self)
    }
}

impl<S: Serializer> Serialize<S> for usize {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.i64(*self as i64)
    }
}

impl<S: Serializer> Serialize<S> for f32 {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.f64(*self as f64)
    }
}

impl<S: Serializer> Serialize<S> for f64 {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.f64(*self)
    }
}

impl<S: Serializer> Serialize<S> for bool {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.bool(*self)
    }
}

impl<S: Serializer, SERIALIZE: Serialize<S>> Serialize<S> for [SERIALIZE] {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_array();
        for value in self {
            serializer.value(value);
        }
        serializer.end_array();
    }
}

impl<S: Serializer, SERIALIZE: Serialize<S>, const SIZE: usize> Serialize<S> for [SERIALIZE; SIZE] {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_array();
        for value in self {
            serializer.value(value);
        }
        serializer.end_array();
    }
}

impl<S: Serializer, SERIALIZE: Serialize<S>> Serialize<S> for Vec<SERIALIZE> {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_array();
        for value in self {
            serializer.value(value);
        }
        serializer.end_array();
    }
}

impl<S: Serializer, STRING: std::ops::Deref<Target = str>, V: Serialize<S>> Serialize<S>
    for HashMap<STRING, V>
{
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        for (key, value) in self.iter() {
            serializer.property(key);
            serializer.value(value);
        }
        serializer.end_object();
    }
}

impl<S: Serializer, SERIALIZE: Serialize<S>> Serialize<S> for Option<SERIALIZE> {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        if let Some(s) = self {
            s.serialize(serializer);
        } else {
            serializer.null()
        }
    }
}

/*
// Todo: Implement more tuples using a macro
impl<S: Serializer, A: Serialize<S>> Serialize<S> for (A,) {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("i");
        serializer.value(&self.0);
        serializer.end_object();
    }
}

impl<S: Serializer, A: Serialize<S>, B: Serialize<S>> Serialize<S> for (A, B) {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("i0");
        serializer.property("i1");
        serializer.end_object();
    }
}

impl<S: Serializer, A: Serialize<S>, B: Serialize<S>, C: Serialize<S>> Serialize<S> for (A, B, C) {
    #[inline]
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("i0", &self.0);
        serializer.property("i2", &self.1);
        serializer.property("i2", &self.1);
        serializer.end_object();
    }
}
*/

impl<S: Serializer, A: Serialize<S>> Serialize<S> for &A {
    fn serialize(&self, serializer: &mut S) {
        A::serialize(&self, serializer)
    }
}
