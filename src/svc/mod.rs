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
use serde::{Serialize};

use std::fmt::Debug;

use std::sync::Arc;

pub trait State: Clone + Debug + Serialize + DeserializeOwned + Sized + Send {
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
    fn get_state(&self) -> Option<Arc<ST>>;

    // TODO: add to docs.
    // Rationale: Controller can be re-set into operation with deserialized state by parent controller.
    // I currently do not support un-setting controller. This contract might come in handy in the future.
    fn set_state(&mut self, s: Arc<ST>);

    fn is_state_ready(&self) -> bool;
}

pub trait Pilot /*<ST: State, C : Controller<ST>>*/ {
    fn is_live(&self) -> bool;
}

mod simple_impl;
