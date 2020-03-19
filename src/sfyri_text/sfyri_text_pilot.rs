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

use crate::sfyri_text::sfyri_text_state::SfyriTextState;
use crate::svc::Pilot;
use crate::sfyri_text::sfyri_text_controller::SfyriTextController;

struct SfyriTextPilot {}

impl Pilot<SfyriTextController> for SfyriTextPilot {
    fn is_live(&self) -> bool {
        unimplemented!()
    }
}