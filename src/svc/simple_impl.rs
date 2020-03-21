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

use crate::svc::{Controller, Pilot, State};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct SimplePilotImpl<ForwardMsg: Send, BackMsg: Send> {
    s: Sender<ForwardMsg>,
    r: Receiver<BackMsg>,
    id: usize,
}

impl<ForwardMsg: Send, BackMsg: Send> SimplePilotImpl<ForwardMsg, BackMsg> {
    fn new(s: Sender<ForwardMsg>, r: Receiver<BackMsg>, id: usize) -> Self {
        SimplePilotImpl { s, r, id }
    }

    pub fn try_recv(&self) -> Result<BackMsg, TryRecvError> {
        self.r.try_recv()
    }
}

impl<ForwardMsg: Send, BackMsg: Send> Pilot for SimplePilotImpl<ForwardMsg, BackMsg> {
    fn is_live(&self) -> bool {
        unimplemented!()
    }
}

pub struct SimplePilotDesc<ForwardMsg: Send, BackMsg: Send> {
    s: Sender<BackMsg>,
    r: Receiver<ForwardMsg>,
    id: usize,
}

impl<ForwardMsg: Send, BackMsg: Send> SimplePilotDesc<ForwardMsg, BackMsg> {
    pub fn new(s: Sender<BackMsg>, r: Receiver<ForwardMsg>, id: usize) -> Self {
        SimplePilotDesc { s, r, id }
    }
}

//ST : State, C : Controller<ST>,
pub struct SimplePilotManagerImpl<ForwardMsg: Send, BackMsg: Send> {
    p: RefCell<HashMap<usize, SimplePilotDesc<ForwardMsg, BackMsg>>>,
    next_pilot_id: RefCell<usize>,
}

impl<ForwardMsg: Send, BackMsg: Send>
    SimplePilotManagerImpl<ForwardMsg, BackMsg>
{
    pub fn new() -> Self {
        SimplePilotManagerImpl {
            p: RefCell::new(HashMap::new()),
            next_pilot_id: RefCell::new(0)
        }
    }
}

//ST : State, C : Controller<ST>,
impl<ForwardMsg: Send, BackMsg: Send>
    SimplePilotManagerImpl<ForwardMsg, BackMsg>
{
    pub fn get_pilot(&self) -> SimplePilotImpl<ForwardMsg, BackMsg> {
        let (s, r) = crossbeam_channel::unbounded::<ForwardMsg>();
        let (sb, rb) = crossbeam_channel::unbounded::<BackMsg>();

        let id = *self.next_pilot_id.borrow();
        *self.next_pilot_id.borrow_mut() += 1;

        self.p
            .borrow_mut()
            .insert(id, SimplePilotDesc::new(sb, r, id));

        SimplePilotImpl::new(s, rb, id)
    }
}
