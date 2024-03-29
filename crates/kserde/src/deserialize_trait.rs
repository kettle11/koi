use std::borrow::Cow;

/// If a value returns `None` then it should be assumed that the deserializer is
/// no longer in a valid state.
pub trait Deserializer<'a> {
    type Context;
    fn string(&mut self) -> Option<Cow<'a, str>>;
    fn bool(&mut self) -> Option<bool>;
    fn i64(&mut self) -> Option<i64>;
    fn f64(&mut self) -> Option<f64>;
    fn any<'b>(&'b mut self) -> Option<AnyValue<'a>>;

    // I'd prefer the rest of this to be a different trait that
    // borrows from the deserializer, but I couldn't figure out
    // how to make that work without generic associated types,
    // so these functions are here instead.
    fn begin_object(&mut self) -> bool;
    fn end_object(&mut self);

    /// When this returns `None` we're at the end of the object or an error was encountered.
    /// The name of the property is returned.
    fn has_property(&mut self) -> Option<Cow<'a, str>>;

    fn begin_array(&mut self) -> bool;
    fn end_array(&mut self);

    /// When this returns `None` we're at the end of the array or an error was encountered.
    fn has_array_value(&mut self) -> bool;
    fn get_context_mut(&mut self) -> &mut Self::Context;
}

pub trait Deserialize<'a, D: Deserializer<'a>>: Sized {
    fn deserialize(deserializer: &mut D) -> Option<Self>;
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for String {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.string().map(|s| s.to_string())
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Cow<'a, str> {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.string()
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for u8 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for u16 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for u32 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for u64 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for i8 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for i16 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for i32 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|v| v as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for usize {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.i64().map(|i| i as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for f32 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.f64().map(|f| f as _)
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for f64 {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.f64()
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for bool {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.bool()
    }
}

impl<'a, D: Deserializer<'a>, T: Deserialize<'a, D>> Deserialize<'a, D> for Vec<T> {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let mut vec = Vec::new();
        deserializer.begin_array().then(|| {})?;
        while deserializer.has_array_value() {
            vec.push(T::deserialize(deserializer)?)
        }
        deserializer.end_array();
        Some(vec)
    }
}

impl<'a, D: Deserializer<'a>, T: Deserialize<'a, D>> Deserialize<'a, D>
    for std::collections::HashMap<String, T>
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let mut hash_map = std::collections::HashMap::new();
        deserializer.begin_object().then(|| {})?;
        while let Some(key) = deserializer.has_property() {
            let t = T::deserialize(deserializer)?;
            hash_map.insert(key.to_string(), t);
        }
        deserializer.end_object();
        Some(hash_map)
    }
}

impl<'a, D: Deserializer<'a>, T: Deserialize<'a, D>, const COUNT: usize> Deserialize<'a, D>
    for [T; COUNT]
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_array().then(|| {})?;

        // This implementation is pretty funky.
        // It feels like this behavior should be handled by something from the standard library.
        let mut a = std::mem::MaybeUninit::<[T; COUNT]>::uninit();
        let result = unsafe {
            for i in 0..COUNT {
                if deserializer.has_array_value() {
                    let t = T::deserialize(deserializer);

                    if let Some(t) = t {
                        a.as_mut_ptr().cast::<T>().add(i).write(t);
                        continue;
                    }
                }

                // If this deserialization fails early then
                // we need to drop all the previous elements before returning.
                for j in 0..i {
                    std::ptr::drop_in_place(a.as_mut_ptr().cast::<T>().add(j))
                }
                return None;
            }
            let a = a.assume_init();

            // This is needed to consume the end of the array.
            if !deserializer.has_array_value() {
                Some(a)
            } else {
                None
            }
        };
        deserializer.end_array();
        result
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for () {
    fn deserialize(_deserializer: &mut D) -> Option<Self> {
        Some(())
    }
}

// Todo: These tuple implementations should be implemented with a macro and for more of them.
impl<'a, D: Deserializer<'a>, A: Deserialize<'a, D>> Deserialize<'a, D> for (A,) {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let a;
        deserializer.begin_object();

        if deserializer.has_property().map_or(false, |p| p == "a") {
            a = A::deserialize(deserializer)?;
        } else {
            return None;
        }

        deserializer.end_object();
        Some((a,))
    }
}

impl<'a, D: Deserializer<'a>, A: Deserialize<'a, D>, B: Deserialize<'a, D>> Deserialize<'a, D>
    for (A, B)
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let a;
        let b;
        deserializer.begin_object();

        if deserializer.has_property().map_or(false, |p| p == "a") {
            a = A::deserialize(deserializer)?;
        } else {
            return None;
        }

        if deserializer.has_property().map_or(false, |p| p == "b") {
            b = B::deserialize(deserializer)?;
        } else {
            return None;
        }

        deserializer.end_object();
        Some((a, b))
    }
}

impl<
        'a,
        D: Deserializer<'a>,
        A: Deserialize<'a, D>,
        B: Deserialize<'a, D>,
        C: Deserialize<'a, D>,
    > Deserialize<'a, D> for (A, B, C)
{
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let a;
        let b;
        let c;
        deserializer.begin_object();

        if deserializer.has_property().map_or(false, |p| p == "a") {
            a = A::deserialize(deserializer)?;
        } else {
            return None;
        }

        if deserializer.has_property().map_or(false, |p| p == "b") {
            b = B::deserialize(deserializer)?;
        } else {
            return None;
        }

        if deserializer.has_property().map_or(false, |p| p == "c") {
            c = C::deserialize(deserializer)?;
        } else {
            return None;
        }

        deserializer.end_object();

        Some((a, b, c))
    }
}

// Probably should have some sort of slice deserialization here,

pub enum AnyValue<'a> {
    String(std::borrow::Cow<'a, str>),
    Bool(bool),
    Number(f64),
    Object,
    Array,
    Null,
}

impl<'a> AnyValue<'a> {
    pub fn string(self) -> Option<Cow<'a, str>> {
        match self {
            Self::String(a) => Some(a),
            _ => None,
        }
    }

    pub fn number(self) -> Option<f64> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }
}
