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

use crate::interface::interface_msg::{InterfaceBackMsg, InterfaceMsg};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use cursive::Cursive;
use std::string::ToString;
use std::thread;
use std::sync::{Arc, Mutex};

pub enum InterfaceWorkerResult {
    Quit,
}

pub struct InterfaceWorker {
    receiver: Receiver<InterfaceMsg>,
    sender: Sender<InterfaceBackMsg>,
    siv: Cursive,
    tick: u64,
}

impl InterfaceWorker {
    pub fn start(
        receiver: Receiver<InterfaceMsg>,
        sender: Sender<InterfaceBackMsg>,
    ) -> thread::JoinHandle<InterfaceWorkerResult> {
        thread::Builder::new()
            .name("interface_worker".to_string())
            .spawn(move || {
                let mut siv = Cursive::default();
                let mut worker = InterfaceWorker {
                    receiver,
                    sender,
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
        while (self.siv.is_running()) {
            self.tick += 1;
            /// here we block on input.
            self.siv.step();

            self.post_input_tick();

            match self.receiver.try_recv() {
                Err(TryRecvError::Empty) => {}
                Err(e) => debug!("worker main got error {:?}", e),

                Ok(msg) => match msg {
                    InterfaceMsg::ShutDown => {
                        debug!("received shutdown signal.");
                        self.siv.quit();
                    }
                    // InterfaceMsg::AddView{ t } => self.add_view(),
                    _ => {
                        debug!("unhandled InterfaceMsg {:?}", msg);
                    }
                },
            }

            self.siv.refresh();
        }

        debug!("interface worker thread ends.");
        InterfaceWorkerResult::Quit
    }

    // fn add_view(&mut self) {
    //
    //     let buffer: Box<dyn Buffer> = Box::new(DefaultBuffer::empty());
    //     let arc = Arc::new(Mutex::new(buffer));
    //     let mut view = SfyriTextView::new(arc);
    //
    //     self.siv.add_fullscreen_layer(view);
    // }
}
