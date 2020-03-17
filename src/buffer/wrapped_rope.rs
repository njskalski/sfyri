/*
This file is part of Sfyri.

Sfyri is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Sfyri is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Foobar.  If not, see <https://www.gnu.org/licenses/>.
*/

use ropey::Rope;
use serde::de::{self, Visitor};
use serde::export::fmt;
use serde::export::Formatter;
use serde::{Deserializer, Serializer};

#[derive(Clone)]
pub struct WrappedRope {
    pub r: Rope,
}

impl WrappedRope {
    pub fn empty() -> Self {
        WrappedRope { r: Rope::new() }
    }
}

// I have no idea what I'm doing: https://serde.rs/impl-deserialize.html

impl serde::Serialize for WrappedRope {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.r.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for WrappedRope {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(WrappedRopeVisitor)
    }
}

struct WrappedRopeVisitor;

impl<'de> de::Visitor<'de> for WrappedRopeVisitor {
    type Value = WrappedRope;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("utf-8 string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(WrappedRope {
            r: Rope::from_str(value),
        })
    }
}

impl From<Rope> for WrappedRope {
    fn from(r: Rope) -> Self {
        WrappedRope { r }
    }
}
