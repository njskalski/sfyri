/* Portions of this file might have been copied from https://github.com/njskalski/sly-editor/
   I honestly don't remember.
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
along with Foobar.  If not, see <https://www.gnu.org/licenses/>.
*/

use ropey::Rope;
use crate::buffer::buffer_trait::{Buffer, BufferContent, BufferType};
use crate::edit_event::EditEvent;

pub struct RopeBufferContent {
    rope : Rope
}

pub struct DefaultBuffer {
    history: Vec<RopeBufferContent>,
    pointer: usize
}

impl DefaultBuffer {
    pub fn empty() -> Self {
        DefaultBuffer {
            history : vec![RopeBufferContent{ rope : Rope::new() }],
            pointer: 0
        }
    }

    fn current(&self) -> &RopeBufferContent {
        &self.history[self.pointer]
    }

    fn current_mut(&mut self) -> &mut RopeBufferContent {
        &mut self.history[self.pointer]
    }
}

impl Buffer for DefaultBuffer {
    fn get_type(&self) -> BufferType {
        BufferType::ReadWrite
    }

    fn get_content(&self) -> Box<&BufferContent> {
        Box::new(self.current())
    }

    fn get_content_mut(&mut self) -> Box<&mut BufferContent> { Box::new(self.current_mut()) }

    fn check_concord_slow(&self) -> bool {
        unimplemented!()
    }

    fn is_persisted(&self) -> bool {
        unimplemented!()
    }

    fn can_undo(&self) -> bool {
        self.pointer > 0
    }

    fn can_redo(&self) -> bool {
        self.pointer < (self.history.len() - 1)
    }

    fn undo(&mut self) {
        unimplemented!()
    }

    fn redo(&mut self) {
        unimplemented!()
    }

    fn submit_events(&mut self, events: Vec<EditEvent>) {
        debug!(target: "default_buffer", "got events {:?}", events);
        let (new_content, num_common_lines) = apply_events(&self.current().rope, &events);
        let rope = new_content.clone(); // O(1)

        self.history.truncate(self.pointer + 1); //droping redo's
        self.history.push(RopeBufferContent { rope : new_content });
        self.pointer += 1;
    }
}

impl BufferContent for RopeBufferContent {
    fn as_string(&self) -> String {
        self.rope.to_string()
    }

    fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    fn line_to_char(&self, line_idx : usize) -> usize {
        self.rope.line_to_byte(line_idx)
    }

    fn char_to_line(&self, char_idx : usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    fn line(&self, line_idx: usize) -> String {
        self.rope.line(line_idx).to_string()
    }

    fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }
}

// Applies events to RopeBasedContent producing new one, and returning *number of lines common* to
// both new and original contents.
// Now events are applied one after another in order they were issued.
//TODO in some combinations offsets should be recomputed. But I expect no such combinations appear.
// I should however check it just in case.
fn apply_events(old_lines : &Rope, events: &Vec<EditEvent>) -> (Rope, usize) {
    let mut new_lines: Rope = old_lines.clone();

    // Offset is in CHARS, and since it's common, it's valid in both new and old contents.
    let mut first_change_pos = new_lines.len_chars();

    for event in events {
        match event {
            &EditEvent::Insert { ref offset, ref content } => {
                first_change_pos = std::cmp::min(first_change_pos, *offset);
                new_lines.insert(*offset, content);
            }
            &EditEvent::Change { ref offset, ref length, ref content } => {
                first_change_pos = std::cmp::min(first_change_pos, *offset);
                new_lines.remove(*offset..(*offset + *length));
                new_lines.insert(*offset, content);
            }
            _ => debug!(target: "apply_events", "event {:?} not supported yet", event),
        }
    }

    // If first_change_pos is 0 (literally first character of file), obviously there are no
    // common lines betwen old and new version.
    // In other case (first_change_pos > 0), we ask of line_of_first_change. If it's not the first
    // one, we can save all lines before it.
    let num_common_lines = if first_change_pos == 0 {
        0
    } else {
        let line_of_first_change = old_lines.char_to_line(first_change_pos);
        if line_of_first_change > 0 {
            line_of_first_change - 1
        } else {
            0
        }
    };

    (new_lines, num_common_lines)
}
