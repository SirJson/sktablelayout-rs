#![feature(test)]

extern crate sktablelayout;
extern crate test;

use sktablelayout::*;

#[bench]
fn impose2x3(b: &mut test::Bencher) {
    // We only test the speed of layout calculation here, not
    // the overhead of communicating the layout back to the client.
    let mut engine = TableLayout::new();
    engine.with_cell(CellProperties::new()
                    .anchor_right()
                    .anchor_bottom()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_cell(CellProperties::new()
                    .anchor_top()
                    .anchor_left()
                    .expand_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_cell(CellProperties::new()
                    .anchor_right()
                    .expand_horizontal()
                    .fill_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_row();
    engine.with_cell(CellProperties::new()
                    .colspan(3)
                    .expand_vertical()
                    .anchor_bottom()
                    .fill_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    b.iter(|| engine.impose(test::black_box(320.0), test::black_box(240.0)))
}
