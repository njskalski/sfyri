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

use crate::interface::interface_pilot::{InterfaceBackMsg, InterfaceMsg, InterfacePilot};

use crossbeam_channel::{Receiver, Sender, TryRecvError};
use cursive::Cursive;
use std::string::ToString;

use crate::interface::interface_state::InterfaceState;
use std::sync::Arc;
use std::thread;

pub enum InterfaceWorkerResult {
    Quit,
}

pub struct InterfaceWorker {
    ip: InterfacePilot,
    r: Receiver<Arc<InterfaceState>>,
    siv: Cursive,
    tick: u64,
}

impl InterfaceWorker {
    pub fn start(
        ip: InterfacePilot,
        r: Receiver<Arc<InterfaceState>>,
    ) -> thread::JoinHandle<InterfaceWorkerResult> {
        thread::Builder::new()
            .name("interface_worker".to_string())
            .spawn(move || {
                let siv = Cursive::default();
                let mut worker = InterfaceWorker {
                    ip,
                    r,
                    siv,
                    tick: 0,
                };
                worker.main()
            })
            .unwrap()
    }

    fn post_input_tick(&self) {
        debug!("{}:\t\t\t post input tick", self.tick);
    }

    pub fn main(&mut self) -> InterfaceWorkerResult {
        while self.siv.is_running() {
            self.tick += 1;
            /// here we block on input.
            self.siv.step();

            self.post_input_tick();

            match self.ip.try_recv() {
                Err(TryRecvError::Empty) => {}
                Err(e) => debug!("worker main got error {:?}", e),

                Ok(msg) => match msg {
                    InterfaceBackMsg::ShutDown => {
                        debug!("received shutdown signal.");
                        self.siv.quit();
                    }
                    // InterfaceMsg::UpdateState {state } => {
                    //     self.is = state;
                    // }
                    other => debug!("Ignoring InterfaceMsg of {:?}", other),
                },
            }

            match self.r.recv() {
                Ok(state) => self.process_state(state),
                Err(e) => warn!("We got an arror receiving state from Interface: {:?}", e)
            }

            self.siv.refresh();
        }

        debug!("interface worker thread ends.");
        InterfaceWorkerResult::Quit
    }

    fn process_state(&mut self,state : Arc<InterfaceState>) {

    }
}
