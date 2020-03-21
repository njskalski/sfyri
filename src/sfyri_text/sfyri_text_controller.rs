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
use crate::svc::simple_impl::SimplePilotManagerImpl;
use crate::svc::Controller;
use crossbeam_channel::{Receiver, Sender};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SfyriTextController {
    s: Option<Arc<SfyriTextState>>,
    pub pc: SimplePilotManagerImpl<SfyriPilotMsg, SfyriPilotBackMsg>,
}

impl SfyriTextController {
    pub fn empty() -> Self {
        SfyriTextController {
            s: Some(Arc::new(SfyriTextState::empty())),
            pc: SimplePilotManagerImpl::new(),
        }
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
