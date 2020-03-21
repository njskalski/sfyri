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

use crate::buffer::apply_events::apply_events;
use crate::buffer::buffer_state::BufferState;

use crate::edit_event::EditEvent;
use crate::svc::Controller;



use std::sync::Arc;

pub struct BufferController {
    s: Option<Arc<BufferState>>,
}

impl BufferController {
    pub fn new() -> Self {
        BufferController {
            s: Some(Arc::new(BufferState::empty())),
        }
    }

    fn advance_state(&mut self, events: &Vec<EditEvent>) {
        if self.s.is_none() {
            warn!("Buffer controller called to advance on empty buffer state! Initializing with empty state to avoid crash. This should not have happened.");
            self.s = Some(Arc::new(BufferState::empty()))
        }

        let old_state = self.s.take().unwrap();

        let old_rope = old_state.get_rope();
        let (new_rope, _num_common_lines) = apply_events(&old_rope, events);

        let new_state = BufferState::new(new_rope, None, Some(old_state));

        self.s = Some(Arc::new(new_state))
    }
}

impl Controller<BufferState> for BufferController {
    fn get_state(&self) -> Option<Arc<BufferState>> {
        self.s.clone()
    }

    fn set_state(&mut self, state_op: Arc<BufferState>) {
        self.s = Some(state_op);
    }

    fn is_state_ready(&self) -> bool {
        self.s.is_some()
    }
}
