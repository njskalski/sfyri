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

use crate::edit_event::EditEvent;

pub enum BufferType {
    ReadWrite,
    ReadOnly,
}

pub trait Buffer {
    fn get_type(&self) -> BufferType;
    fn get_content(&self) -> Box<&BufferContent>;
    fn get_content_mut(&mut self) -> Box<&mut BufferContent>;

    fn check_concord_slow(&self) -> bool;
    fn is_persisted(&self) -> bool;
    fn can_undo(&self) -> bool;
    fn can_redo(&self) -> bool;

    fn undo(&mut self);
    fn redo(&mut self);

    fn submit_events(&mut self, events: Vec<EditEvent>);
}

pub trait BufferContent {
    fn as_string(&self) -> String;
    fn len_lines(&self) -> usize;
    fn line_to_char(&self, line_idx: usize) -> usize;
    fn char_to_line(&self, char_idx: usize) -> usize;
    fn len_chars(&self) -> usize;
    fn line(&self, line_idx: usize) -> String;
}
