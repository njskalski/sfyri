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

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate ropey;
#[macro_use]
extern crate serde;

use std::borrow::BorrowMut;

mod buffer;
mod cursor_set;
mod edit_event;
mod interface;
mod logger;
mod sfyri_text;
mod svc;
mod view;
mod view_type;

use interface::interface_controller::InterfaceController;
use logger::sfyri_logger;

fn main() {
    if sfyri_logger::init_logger().is_err() {
        eprintln!("FATAL: unable to initialize logger, shutting down.");
        return;
    }

    let yml = clap::load_yaml!("clap.yml");
    let mut app = clap::App::from_yaml(yml)
        .author("NJ Skalski <gitstuff@s5i.ch>")
        .long_version(crate_version!());

    let matches = app.clone().get_matches();

    if matches.is_present("help") {
        app.write_long_help(std::io::stdout().borrow_mut())
            .err()
            .map(|e| eprint!("FATAL: Error displaying help, due {}", e));
        return;
    }

    let mut ic: InterfaceController = InterfaceController::new();

    ic.tmp_add_view();

    ic.tmp_join();

    debug!("goodbye!");
}
