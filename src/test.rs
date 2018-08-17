use ::*;
#[test]
fn expanding_layout() {
    let mut engine = TableLayout::new();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 0.0);
                        assert_eq!(y, 0.0);
                        // these are expand, not fill, so the
                        // cell takes up extra space but the
                        // child item actually doesn't use it
                        assert_eq!(w, 64.0);
                        assert_eq!(h, 64.0);
                    }))
                    .anchor_right()
                    .anchor_bottom()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        // first column is neither expand nor
                        // fill horizontal, so should be packed
                        // as preferred width
                        assert_eq!(x, 64.0);
                        assert_eq!(y, 0.0);
                        // these are expand, not fill, so the
                        // cell takes up extra space but the
                        // child item actually doesn't use it
                        assert_eq!(w, 64.0);
                        assert_eq!(h, 64.0);
                    }))
                    .anchor_top()
                    .anchor_left()
                    .expand_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(y, 0.0);
                        assert_eq!(h, 64.0);
                    }))
                    .anchor_right()
                    .expand_horizontal()
                    .fill_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_row();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 0.0);
                        assert_eq!(y, 240.0 - 64.0);
                        assert_eq!(w, 320.0);
                    }))
                    .colspan(3)
                    .expand_vertical()
                    .anchor_bottom()
                    .fill_horizontal()
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.impose(320.0, 240.0);
}

#[test]
fn shrinking_layout() {
    let mut engine = TableLayout::new();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 0.0);
                        assert_eq!(y, 0.0);
                        assert_eq!(w, 16.0);
                        assert_eq!(h, 16.0);
                    }))
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 16.0);
                        assert_eq!(y, 0.0);
                        assert_eq!(w, 16.0);
                        assert_eq!(h, 16.0);
                    }))
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.with_row();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 0.0);
                        assert_eq!(y, 16.0);
                        assert_eq!(w, 32.0);
                        assert_eq!(h, 16.0);
                    }))
                    .colspan(2)
                    .preferred_size(Size{width: 64.0, height: 64.0}));
    engine.impose(32.0, 32.0);
}

#[test]
fn centered_layout() {
    let mut engine = TableLayout::new();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 16.0);
                        assert_eq!(y, 16.0);
                        assert_eq!(w, 32.0);
                        assert_eq!(h, 32.0);
                    }))
                    .anchor_horizontal_center()
                    .anchor_vertical_center()
                    .expand()
                    .preferred_size(Size{width: 32.0, height: 32.0}));
    engine.impose(64.0, 64.0);
}

#[test]
fn padded_big_cell() {
    let mut engine = TableLayout::new();
    engine.with_cell(CellProperties::new()
                    .callback(Box::new(|x, y, w, h| {
                        println!("{} {} {} {}", x, y, w, h);
                        assert_eq!(x, 16.0);
                        assert_eq!(y, 16.0);
                        assert_eq!(w, 32.0);
                        assert_eq!(h, 32.0);
                    }))
                    .expand()
                    .fill()
                    .padding_all(16.0));
    engine.impose(64.0, 64.0);
}
