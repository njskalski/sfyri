/* This is a fork of https://github.com/njskalski/sly-editor/src/sly_text_view.rs
If so, the original is licensed under Apache license. All subsequent changes are GPLv3.

*/

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

use crate::buffer::buffer_state::BufferState;

use crate::sfyri_text::sfyri_text_state::SfyriTextState;
use core::borrow::Borrow;

use cursive::event::{Event, EventResult};

use cursive::theme::{ColorStyle, Effect};

use cursive::vec::Vec2;
use cursive::view::View;

use cursive::views::IdView;

use cursive::Printer;

use std::cmp;

use std::string::ToString;
use std::sync::Arc;

use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

const INDEX_MARGIN: usize = 1;
const PAGE_WIDTH: usize = 80;

// This is suppose to be a view of view, that is to visualize state.
pub struct SfyriTextView {
    s: Arc<SfyriTextState>,
    position: Vec2,               // position of upper left corner of view in file
    last_view_size: Option<Vec2>, //not sure if using properly
}

impl SfyriTextView {
    pub fn new(s: Arc<SfyriTextState>) -> IdView<Self> {
        let _syntax_highlighting: bool = false;

        let view = SfyriTextView {
            s,
            position: Vec2::new(0, 0),
            last_view_size: None,
        };

        IdView::new("text_view", view)
    }

    pub fn update_state(&mut self, s: Arc<SfyriTextState>) {
        self.s = s;
    }
}

//TODO(njskalski): handle too small space.
impl View for SfyriTextView {
    fn draw(&self, printer: &Printer) {
        let buffer: &BufferState = self.s.buffer.borrow();
        let index_length = buffer.len_lines().to_string().len();
        let _cursors = &self.s.cursor_set;

        let view_size = self.last_view_size.expect("view size not known.");

        //index + INDEX_MARGIN ----------------------------------------------------------------
        for line_no in
            (self.position.y)..(cmp::min(buffer.len_lines(), self.position.y + view_size.y))
        {
            let mut x: usize = 0;

            let y = line_no - self.position.y;
            let line_desc = (line_no + 1).to_string();
            let local_index_length = line_desc.len(); //logarithm? never heard of it.

            printer.with_color(ColorStyle::secondary(), |printer| {
                for _ in 0..(index_length - local_index_length) {
                    printer.print((x, y), " ");
                    x += 1;
                }
                printer.print((x, y), &line_desc);
                x += local_index_length;
                for _ in 0..INDEX_MARGIN {
                    printer.print((x, y), " ");
                    x += 1;
                }
            });

            assert!(x == index_length + INDEX_MARGIN);
        }
        // end of index + INDEX_MARGIN --------------------------------------------------------

        //line --------------------------------------------------------------------------------

        for line_no in
            (self.position.y)..(cmp::min(buffer.len_lines(), self.position.y + view_size.y))
        {
            let y = line_no - self.position.y;
            let line_offset = &buffer.line_to_char(line_no);
            // TODO never request for more line than the view can hold.
            // TODO add test with extremely long line.
            let line = &buffer.line(line_no);

            //this allow a cursor *after* the last character. It's actually needed.
            let add = if line_no == buffer.len_lines() - 1 {
                1
            } else {
                0
            };

            let line_char_count = line.chars().count();
            for char_idx in 0..(line_char_count + add) {
                let char_offset = line_offset + char_idx;

                let special_char = false;
                let symbol: &str = if line_char_count > char_idx {
                    let c = line[char_idx..].graphemes(true).next().unwrap();

                    c
                } else {
                    " "
                };

                let color_style: ColorStyle = if self.s.has_cursor_at(char_offset) {
                    ColorStyle::highlight()
                } else {
                    if char_idx <= 80 && !special_char {
                        let some_color = ColorStyle::primary();

                        some_color
                    } else {
                        ColorStyle::secondary()
                    }
                };

                let effect = Effect::Simple;

                printer.with_color(color_style, |printer| {
                    printer.with_effect(effect, |printer| {
                        printer.print((char_idx + index_length + INDEX_MARGIN, y), symbol);
                    });
                });
            }
        }
        //end of line ------------------------------------------------------------------------
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.last_view_size = Some(constraint);
        //        debug!("got constraint {:?}", constraint);
        constraint //now we just take whole available space
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let mut consumed = true;
        match event {
            // Event::Char(c) => {
            //     self.s.add_text(&c.to_string());
            // }
            // Event::Key(Key::Enter) => {
            //     self.s.add_text(&'\n'.to_string());
            // }
            // Event::Key(Key::Backspace) => {
            //     self.s.backspace();
            // }
            // Event::Key(Key::Left) => {
            //     self.s.arrow_left();
            // }
            // Event::Key(Key::Right) => {
            //     self.s.arrow_right();
            // }
            // Event::Key(Key::Up) => {
            //     self.s.arrow_up();
            // }
            // Event::Key(Key::Down) => {
            //     self.s.arrow_down();
            // }
            //TODO(njskalski): implement scrolling up
            //            Event::Key(Key::PageUp) => {
            //                self.s.page_up()
            //          }
            //TODO(njskalski): implement scrolling down
            //            Event::Key(Key::PageDown) => {
            //                self.s.page_down();
            //            }
            _ => {
                debug!("unhandled event (in sly_text_view) {:?}", event);
                consumed = false;
            }
        };
        if consumed {
            EventResult::Consumed(None)
        } else {
            EventResult::Ignored
        }
    }
}
