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

use crate::interface::interface_state::InterfaceState;
use crate::interface::interface_worker::{InterfaceWorker, InterfaceWorkerResult};
use crate::svc::Controller;

use crate::interface::interface_pilot::{InterfaceBackMsg, InterfaceMsg, InterfacePilot};
use std::sync::Arc;
use std::thread;

use crate::sfyri_text::sfyri_text_controller::SfyriTextController;
use crate::svc::simple_impl::SimplePilotManagerImpl;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct InterfaceController {
    state: Arc<InterfaceState>,
    handle: Option<thread::JoinHandle<InterfaceWorkerResult>>,
    ss: Sender<Arc<InterfaceState>>,
    pc: SimplePilotManagerImpl<InterfaceMsg, InterfaceBackMsg>,
    tmp_textview_controller: SfyriTextController,
}

impl InterfaceController {
    pub fn new() -> Self {
        let pc = SimplePilotManagerImpl::<InterfaceMsg, InterfaceBackMsg>::new();
        let (ss, sr) = crossbeam_channel::unbounded::<Arc<InterfaceState>>();
        let handle = InterfaceWorker::start(pc.get_pilot(), sr);

        InterfaceController {
            state: Arc::new(InterfaceState::new()),
            handle: Some(handle),
            pc,
            ss,
            // TODO remove
            tmp_textview_controller: SfyriTextController::empty(),
        }
    }

    pub fn tmp_join(&mut self) {
        self.handle.take().unwrap().join();
    }
}

impl Controller<InterfaceState> for InterfaceController {
    fn get_state(&self) -> Option<Arc<InterfaceState>> {
        Some(self.state.clone())
    }

    fn set_state(&mut self, s: Arc<InterfaceState>) {
        self.state = s;
    }

    fn is_state_ready(&self) -> bool {
        true
    }
}
