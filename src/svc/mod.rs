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

use std::ops::Deref;
use std::sync::Arc;
use std::borrow::Borrow;
use serde::{Deserialize, Serialize};

pub trait State: Serialize + Sized {}

pub trait Controller<ST: State> {
    fn get_state(&self) -> StateRef<ST>;
}

// It's a wrapper to take away writing capabilities.
#[derive(Clone)]
pub struct StateRef<ST : State> {
    arc : Arc<ST>
}

unsafe impl <ST : State> Send for StateRef<ST> {}

impl <ST : State> StateRef<ST> {
    fn new(arc : Arc<ST>) -> Self {
        StateRef { arc }
    }
}

impl <ST : State> Borrow<ST> for StateRef<ST> {
    fn borrow(&self) -> &ST {
        self.arc.borrow()
    }
}

impl <ST : State>  From<Arc<ST>> for StateRef<ST> {
    fn from(arc: Arc<ST>) -> Self {
        StateRef::new(arc)
    }
}