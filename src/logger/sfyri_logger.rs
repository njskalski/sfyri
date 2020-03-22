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

use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use std::io::stderr;
use std::io::Write;

static LOGGER: SfyriLogger = SfyriLogger;

struct SfyriLogger;

impl log::Log for SfyriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        stderr()
            .flush()
            .err()
            .map(|e| eprintln!("FATAL: unable to flush stderr, due {}", e));
    }
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
