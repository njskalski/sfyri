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

use crate::buffer::wrapped_rope::WrappedRope;

use crate::svc::State;
use ropey::Rope;

use std::sync::Arc;

// Cursors are not part of BufferState, for now.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BufferState {
    w: WrappedRope,
    prev: Option<Arc<BufferState>>,
    next: Option<Arc<BufferState>>,
}

impl State for BufferState {}

impl BufferState {
    pub fn empty() -> Self {
        BufferState {
            w: WrappedRope::empty(),
            prev: None,
            next: None,
        }
    }

    pub fn new(r: Rope, prev: Option<Arc<BufferState>>, next: Option<Arc<BufferState>>) -> Self {
        BufferState {
            w: WrappedRope::from(r),
            prev,
            next,
        }
    }

    // Boring methods:

    pub fn len_chars(&self) -> usize {
        self.w.r.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.w.r.len_lines()
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.w.r.char_to_line(char_idx)
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.w.r.line_to_char(line_idx)
    }

    pub fn get_rope(&self) -> &Rope {
        &self.w.r
    }

    // TODO this should be removed for a bounded line request (?) \
    // OR make it return an iterator over line.
    pub fn line(&self, line_idx: usize) -> String {
        self.w.r.line(line_idx).to_string().clone()
    }
}
