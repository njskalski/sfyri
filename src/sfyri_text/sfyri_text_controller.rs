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

use crate::sfyri_text::sfyri_text_pilot::{SfyriPilotBackMsg, SfyriPilotMsg, SfyriTextPilot};
use crate::sfyri_text::sfyri_text_state::SfyriTextState;
use crate::svc::Controller;
use crossbeam_channel::{Receiver, Sender};
use std::borrow::Borrow;
use std::cell::{RefCell};
use std::collections::{HashMap};
use std::sync::Arc;

struct PilotDesc {
    s: Sender<SfyriPilotBackMsg>,
    r: Receiver<SfyriPilotMsg>,
}

pub struct SfyriTextController {
    s: Option<Arc<SfyriTextState>>,
    p: RefCell<HashMap<usize, PilotDesc>>,
    next_pilot_id: RefCell<usize>, //TODO looping
}

impl SfyriTextController {
    pub fn empty() -> Self {
        SfyriTextController {
            s: Some(Arc::new(SfyriTextState::empty())),
            p: RefCell::new(HashMap::new()),
            next_pilot_id: RefCell::new(0),
        }
    }

    pub fn get_pilot(&self) -> SfyriTextPilot {
        let (s, r) = crossbeam_channel::unbounded::<SfyriPilotMsg>();
        let (sb, rb) = crossbeam_channel::unbounded::<SfyriPilotBackMsg>();

        let pd = PilotDesc { s: sb, r };

        self.p.borrow_mut().insert(*self.next_pilot_id.borrow(), pd);
        *self.next_pilot_id.borrow_mut() += 1;

        SfyriTextPilot::new(s, rb)
    }
}

impl Controller<SfyriTextState> for SfyriTextController {
    fn get_state(&self) -> Option<Arc<SfyriTextState>> {
        unimplemented!()
    }

    fn set_state(&mut self, _s: Arc<SfyriTextState>) {
        unimplemented!()
    }

    fn is_state_ready(&self) -> bool {
        unimplemented!()
    }
}
