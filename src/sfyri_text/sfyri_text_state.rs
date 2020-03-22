/* Portions of this file are copied from https://github.com/njskalski/sly-editor/
If so, the original is licensed under Apache license. All subsequent changes are GPLv3
*/

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

use crate::buffer::buffer_state::BufferState;
use crate::cursor_set::CursorSet;
use crate::edit_event::EditEvent;
use crate::svc::State;

use std::sync::Arc;

// This is supposed to be a serializable state of view
// Impossible. Pointer will never be serializable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfyriTextState {
    // Not sure if anyone except View have any use of buffer (async write?). Will leave it now like this.
    pub buffer: Arc<BufferState>,
    pub cursor_set: CursorSet,
}

impl State for SfyriTextState {}

impl SfyriTextState {
    pub fn new(buffer: Arc<BufferState>) -> Self {
        SfyriTextState {
            buffer,
            cursor_set: CursorSet::single()
        }
    }

    pub fn empty() -> Self {
        SfyriTextState::new(Arc::new(BufferState::empty()))
    }

    /// Returns the position of the cursor in the content string.
    pub fn cursors(&self) -> &CursorSet {
        &self.cursor_set
    }

    // TODO(njskalski): Expand definition for anchor+selection model.
    pub fn has_cursor_at(&self, char_idx: usize) -> bool {
        self.cursor_set.has_anchor_at(char_idx)
    }

    pub fn add_text<S: ToString>(&mut self, text: &S) {
        let mut edit_events: Vec<EditEvent> = self
            .cursor_set
            .set()
            .iter()
            .map(|ref cursor| EditEvent::Insert {
                offset: cursor.a,
                content: text.to_string(),
            })
            .collect();
        edit_events.reverse();

        let text_len =
            unicode_segmentation::UnicodeSegmentation::graphemes(text.to_string().as_str(), true)
                .count();

        self.cursor_set
            .move_right_by(self.buffer.as_ref(), text_len);
    }

    // // TODO(njskalski): fix, test
    // pub fn backspace(&mut self) {
    //     let mut edit_events: Vec<EditEvent> = self
    //         .cursor_set
    //         .set()
    //         .iter()
    //         .filter(|&cursor| cursor.a > 0)
    //         .map(|ref cursor| EditEvent::Change {
    //             offset: cursor.a - 1,
    //             length: 1,
    //             content: "".to_string(),
    //         })
    //         .collect();
    //     edit_events.reverse();
    //
    //     let mut buffer_locked = self.buffer.lock().unwrap();
    //     buffer_locked.submit_events(edit_events);
    //     self.cursor_set.move_left();
    // }
    //
    // pub fn arrow_left(&mut self) {
    //     self.cursor_set.move_left();
    // }
    //
    // pub fn arrow_right(&mut self) {
    //     let buffer_locked = self.buffer.lock().unwrap();
    //     &self.cursor_set.move_right(*buffer_locked.get_content());
    // }
    //
    // pub fn arrow_up(&mut self) {
    //     let buffer_locked = self.buffer.lock().unwrap();
    //     &self
    //         .cursor_set
    //         .move_vertically_by(*buffer_locked.get_content(), -1);
    // }
    //
    // pub fn arrow_down(&mut self) {
    //     let buffer_locked = self.buffer.lock().unwrap();
    //     &self
    //         .cursor_set
    //         .move_vertically_by(*buffer_locked.get_content(), 1);
    // }
}
