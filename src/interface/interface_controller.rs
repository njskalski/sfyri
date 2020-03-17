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
use crate::svc::{Controller, StateRef};
use std::ops::Deref;
use std::sync::Arc;
use std::thread;

use crate::interface::interface_msg::{InterfaceBackMsg, InterfaceMsg};
use crate::view_type::ViewType;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct InterfaceController {
    state: Arc<InterfaceState>,
    handle: Option<thread::JoinHandle<(InterfaceWorkerResult)>>,
    msgs: Sender<InterfaceMsg>,
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
        }
    }

    pub fn tmp_add_view(&mut self) {
        self.msgs.send(InterfaceMsg::AddView {
            t: ViewType::SfyriTextView,
        });
    }

    pub fn tmp_join(&mut self) {
        self.handle.take().unwrap().join();
    }
}

impl Controller<InterfaceState> for InterfaceController {
    fn get_state(&self) -> StateRef<InterfaceState> {
        self.state.clone().into()
    }
}
