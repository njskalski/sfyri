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

#[derive(Clone, Copy, Debug)]
pub enum ViewType {
    SfyriTextView,
    FuzzyView,
}
