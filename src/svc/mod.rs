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

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;

pub trait State: Serialize + DeserializeOwned + Sized {
    fn is_versioned(&self) -> bool {
        false
    }

    fn can_undo(&self) -> bool {
        false
    }

    fn can_redo(&self) -> bool {
        false
    }

    fn get_next(&self) -> Option<Self> {
        None
    }

    fn get_prev(&self) -> Option<Self> {
        None
    }

    fn drop_predecessors(&mut self) {}
    fn drop_successors(&mut self) {}

    fn select(&mut self) {
        self.drop_successors();
        self.drop_predecessors();
    }
}

pub trait Controller<ST: State> {
    // Rationale: Controller can be in incomplete state to render a complete state.
    fn get_state(&self) -> Option<StateRef<ST>>;

    // TODO: add to docs.
    // Rationale: Controller can be re-set into operation with deserialized state by parent controller.
    // I currently do not support un-setting controller. This contract might come in handy in the future.
    fn set_state(&mut self, s: Arc<ST>);

    fn is_state_ready(&self) -> bool;
}

// It's a wrapper to take away writing capabilities.
#[derive(Clone)]
pub struct StateRef<ST: State> {
    arc: Arc<ST>,
}

unsafe impl<ST: State> Send for StateRef<ST> {}

impl<ST: State> StateRef<ST> {
    fn new(arc: Arc<ST>) -> Self {
        StateRef { arc }
    }
}

impl<ST: State> Borrow<ST> for StateRef<ST> {
    fn borrow(&self) -> &ST {
        self.arc.borrow()
    }
}

impl<ST: State> From<Arc<ST>> for StateRef<ST> {
    fn from(arc: Arc<ST>) -> Self {
        StateRef::new(arc)
    }
}
