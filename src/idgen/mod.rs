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
along with Sfyri.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde::de::Unexpected::Str;
use std::marker::Send;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::export::Formatter;
use serde::export::fmt::Error;
use std::fmt;

static next_id : AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq)]
pub struct Id {
    id : usize
}

impl Id {
    pub fn get_next_id() -> Self {
        Id { id : next_id.fetch_add(1, Ordering::Relaxed) }
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.id.to_string()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

unsafe impl Send for Id {}
