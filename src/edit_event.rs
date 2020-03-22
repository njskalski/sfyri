/* Portions of this file might have been copied from https://github.com/njskalski/sly-editor/src/content_provider.rs
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

/// Represents a order to edit a content. Offsets are calculated in CHARS, not bytes.
/// offset is the first character of selection, inclusive.
//TODO(njskalski) secure against overlapping cursors!
#[derive(Debug)]
pub enum EditEvent {
    Insert {
        offset: usize,
        content: String,
    },
    Change {
        offset: usize,
        length: usize,
        content: String,
    },
}
