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

// Code below originates from sly-editor, see github.com/njskalski/sly-editor for Apache licensed
// original.

use crate::edit_event::EditEvent;
use ropey::Rope;

// Applies events to RopeBasedContent producing new one, and returning *number of lines common* to
// both new and original contents.
// Now events are applied one after another in order they were issued.
//TODO in some combinations offsets should be recomputed. But I expect no such combinations appear.
// I should however check it just in case.
pub fn apply_events(old_lines: &Rope, events: &Vec<EditEvent>) -> (Rope, usize) {
    let mut new_lines: Rope = old_lines.clone();

    // Offset is in CHARS, and since it's common, it's valid in both new and old contents.
    let mut first_change_pos = new_lines.len_chars();

    for event in events {
        match event {
            &EditEvent::Insert {
                ref offset,
                ref content,
            } => {
                first_change_pos = std::cmp::min(first_change_pos, *offset);
                new_lines.insert(*offset, content);
            }
            &EditEvent::Change {
                ref offset,
                ref length,
                ref content,
            } => {
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
