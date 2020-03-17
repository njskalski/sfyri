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

use crate::buffer::buffer_state::BufferState;
use crate::svc::{Controller, StateRef};
use std::sync::Arc;

pub struct BufferController {
    s: Arc<BufferState>,
}

impl Controller<BufferState> for BufferController {
    fn get_state(&self) -> StateRef<BufferState> {
        StateRef::from(self.s.clone())
    }

    fn set_state(&mut self, s: Arc<BufferState>) {
        self.s = s;
    }
}
