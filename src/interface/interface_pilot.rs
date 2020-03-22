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

use crate::svc::simple_impl::{SimplePilotImpl, SimplePilotManagerImpl};
use std::sync::Arc;

pub type InterfacePilot = SimplePilotImpl<InterfaceMsg, InterfaceBackMsg>;

#[derive(Clone, Debug)]
pub enum InterfaceMsg {
    PreDrawTick,
    GetState,
}

#[derive(Clone, Debug)]
pub enum InterfaceBackMsg {
    ShutDown,
    Redraw,
}

impl InterfacePilot {}
