// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

use crate::json::Json;

/// Property type captures a single (key,value) pair in a JSON object.
///
/// Where,
/// * **key** is [String] type, defined by JSON specification.
/// * **value** is JSON value.
///
/// Implements [PartialEq] and [PartialOrd] for equality and ordering.
///
/// [string]: std::string::String
/// [PartialEq]: std::cmp::PartialEq
/// [PartialOrd]: std::cmp::PartialOrd
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Property(String, Json);

/// Following inherent methods are self explanatory, typically
/// used to move, or obtain a reference for key or value
/// component of a property.
impl Property {
    #[inline]
    pub fn new<T>(key: T, value: Json) -> Property
    where
        T: ToString,
    {
        Property(key.to_string(), value)
    }

    #[inline]
    pub fn into_key(self) -> String {
        self.0
    }

    #[inline]
    pub fn as_key(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn into_value(self) -> Json {
        self.1
    }

    #[inline]
    pub fn as_value(&self) -> &Json {
        &self.1
    }

    #[inline]
    pub fn as_mut_value(&mut self) -> &mut Json {
        &mut self.1
    }

    #[inline]
    pub fn set_key(&mut self, key: String) {
        self.0 = key;
    }

    #[inline]
    pub fn set_value(&mut self, value: Json) {
        self.1 = value;
    }
}

pub fn upsert_object_key(obj: &mut Vec<Property>, prop: Property) {
    match obj.binary_search_by(|p| p.as_key().cmp(prop.as_key())) {
        Ok(off) => obj[off] = prop,
        Err(off) => obj.insert(off, prop),
    }
}
