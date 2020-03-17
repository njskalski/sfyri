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

//This file is a complete rewrite

use crate::svc::State;
use ropey::Rope;
use serde::{Serializer, Deserializer};
use std::borrow::Borrow;
use crate::buffer::wrapped_rope::WrappedRope;

#[derive(Clone, Serialize, Deserialize)]
pub struct BufferState {
    r : WrappedRope
}

impl State for BufferState {}
