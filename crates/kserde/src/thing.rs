use crate::{AnyValue, Deserialize, Deserializer, JSONDeserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ObjectProperty<'a> {
    pub item: Thing<'a>,
    pub index: usize,
}

#[derive(Debug)]
/// A flexible data structure that everything can deserialize to.
pub enum Thing<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Number(f64),
    Object(HashMap<Cow<'a, str>, ObjectProperty<'a>>),
    Array(Vec<Thing<'a>>),
    Null,
}

#[derive(Debug, Clone)]
pub struct ObjectPropertyOwned {
    pub item: ThingOwned,
    pub index: usize,
}

#[derive(Debug, Clone)]
/// A flexible data structure that everything can deserialize to.
pub enum ThingOwned {
    String(String),
    Bool(bool),
    Number(f64),
    Object(HashMap<String, ObjectPropertyOwned>),
    Array(Vec<ThingOwned>),
    Null,
}

impl<'a> Thing<'a> {
    /// Get an owned instance of this `Thing`.
    pub fn to_owned(&self) -> ThingOwned {
        match self {
            Thing::String(s) => ThingOwned::String(s.to_string()),
            Thing::Bool(b) => ThingOwned::Bool(*b),
            Thing::Number(n) => ThingOwned::Number(*n),
            Thing::Object(h) => {
                let mut new_hash_map = HashMap::with_capacity(h.len());
                for (k, v) in h.iter() {
                    let object_property_owned = ObjectPropertyOwned {
                        item: v.item.to_owned(),
                        index: v.index,
                    };
                    new_hash_map.insert(k.to_string(), object_property_owned);
                }
                ThingOwned::Object(new_hash_map)
            }
            Thing::Array(a) => ThingOwned::Array(a.iter().map(|a| a.to_owned()).collect()),
            Thing::Null => ThingOwned::Null,
        }
    }

    pub fn string(&self) -> Option<&Cow<'a, str>> {
        match self {
            Thing::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn bool(&self) -> Option<bool> {
        match self {
            Thing::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<f64> {
        match self {
            Thing::Number(v) => Some(*v),
            _ => None,
        }
    }

    pub fn object(&self) -> Option<&HashMap<Cow<'a, str>, ObjectProperty<'a>>> {
        match self {
            Thing::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<&Vec<Thing<'a>>> {
        match self {
            Thing::Array(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Thing<'a> {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        Some(match deserializer.any()? {
            AnyValue::Object => {
                let mut items = HashMap::new();
                while let Some(name) = deserializer.has_property() {
                    items.insert(
                        name,
                        ObjectProperty {
                            index: items.len(),
                            item: Thing::deserialize(deserializer)?,
                        },
                    );
                }
                Thing::Object(items)
            }
            AnyValue::Array => {
                let mut items = Vec::new();
                while deserializer.has_array_value() {
                    items.push(Thing::deserialize(deserializer)?);
                }
                Thing::Array(items)
            }
            AnyValue::Number(n) => Thing::Number(n),
            AnyValue::Bool(b) => Thing::Bool(b),
            AnyValue::String(s) => Thing::String(s),
            AnyValue::Null => Thing::Null,
        })
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for ThingOwned {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        <Thing<'a>>::deserialize(deserializer).map(|t| t.to_owned())
    }
}

impl<'a, S: Serializer> Serialize<S> for Thing<'a> {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Object(o) => {
                serializer.begin_object();
                // This allocation and sorting probably isn't ideal,
                // The alternative is to use a sorted HashMap when deserializing
                // into Thing.
                let mut properties: Vec<_> = o.iter().collect();
                properties.sort_by_key(|(_, i)| i.index);
                for (key, value) in o.iter() {
                    serializer.property(key, &value.item);
                }
                serializer.end_object();
            }
            Self::Array(a) => {
                serializer.begin_array();
                for value in a.iter() {
                    serializer.value(value);
                }
                serializer.end_array();
            }
            Self::Number(n) => serializer.f64(*n),
            Self::Bool(b) => serializer.bool(*b),
            Self::String(s) => serializer.string(&s),
            Self::Null => serializer.null(),
        }
    }
}

impl<'a, S: Serializer> Serialize<S> for ThingOwned {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Object(o) => {
                serializer.begin_object();
                // This allocation and sorting probably isn't ideal,
                // The alternative is to use a sorted HashMap when deserializing
                // into Thing.
                let mut properties: Vec<_> = o.iter().collect();
                properties.sort_by_key(|(_, i)| i.index);
                for (key, value) in o.iter() {
                    serializer.property(key, &value.item);
                }
                serializer.end_object();
            }
            Self::Array(a) => {
                serializer.begin_array();
                for value in a.iter() {
                    serializer.value(value);
                }
                serializer.end_array();
            }
            Self::Number(n) => serializer.f64(*n),
            Self::Bool(b) => serializer.bool(*b),
            Self::String(s) => serializer.string(&s),
            Self::Null => serializer.null(),
        }
    }
}

impl<'a> Thing<'a> {
    pub fn from_json(s: &'a str) -> Option<Self> {
        let mut deserializer = JSONDeserializer::new(s);
        Self::deserialize(&mut deserializer)
    }
}

impl<'a> ThingOwned {
    pub fn from_json(s: &'a str) -> Option<Self> {
        let mut deserializer = JSONDeserializer::new(s);
        Self::deserialize(&mut deserializer)
    }
}
