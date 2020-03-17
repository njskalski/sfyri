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

use crate::interface::interface_state::InterfaceState;
use crate::interface::interface_worker::{InterfaceWorker, InterfaceWorkerResult};
use crate::svc::Controller;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;

use crate::interface::interface_msg::{InterfaceBackMsg, InterfaceMsg};
use crate::view_type::ViewType;
use crossbeam_channel::{unbounded, Receiver, Sender};
use crate::buffer::buffer_controller::BufferController;
use std::collections::HashMap;
use crate::buffer::buffer_state::BufferState;
use std::borrow::Borrow;
use crate::sfyri_text::sfyri_text_controller::SfyriTextController;

pub struct InterfaceController {
    state: Arc<InterfaceState>,
    handle: Option<thread::JoinHandle<(InterfaceWorkerResult)>>,
    msgs: Sender<InterfaceMsg>,
    tmp_textview_controller : SfyriTextController
}

impl InterfaceController {
    pub fn new() -> Self {
        let (sender, receiver): (Sender<InterfaceMsg>, Receiver<InterfaceMsg>) = unbounded();
        let (sender_back, receiver_back): (Sender<InterfaceBackMsg>, Receiver<InterfaceBackMsg>) =
            unbounded();

        let handle = InterfaceWorker::start(receiver, sender_back);

        InterfaceController {
            state: Arc::new(InterfaceState::new()),
            handle: Some(handle),
            msgs: sender,
            // TODO remove
            tmp_textview_controller : SfyriTextController::empty()
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
