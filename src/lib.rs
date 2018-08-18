
#[macro_use]
extern crate bitflags;

use std::f32;
use std::cmp::max;
use std::collections::BTreeMap;

/// Rectangle for padding and spacing constraints.
#[derive(Default, Clone, Copy)]
pub struct Rectangle {
    pub top:    f32,
    pub left:   f32,
    pub bottom: f32,
    pub right:  f32,
}

/// Individual size constraint for a cell.
#[derive(Clone)]
pub struct Size {
    pub width:  f32,
    pub height: f32,
}

impl Size {
    pub fn join_max(a: &Size, b: &Size) -> Self {
        Size{
            width:  f32::max(a.width, b.width),
            height: f32::max(a.height, b.height),
        }
    }

    pub fn join_min(a: &Size, b: &Size) -> Self {
        Size{
            width:  f32::min(a.width, b.width),
            height: f32::min(a.height, b.height),
        }
    }

    /// Divides the width and height by a given division level. Used when
    /// a size must be spread across multiple table cells.
    pub fn spread(&self, divisions: f32) -> Self {
        Size{
            width:  self.width / divisions,
            height: self.height / divisions,
        }
    }

    /// Adds padding from a supplied padding rectangle.
    pub fn padded(&self, padding: Rectangle) -> Self {
        Size{
            width:  self.width + padding.left + padding.right,
            height: self.height + padding.top + padding.bottom,
        }
    }

    /// Returns whether this size should fit within another size.
    pub fn within(&self, other: &Size) -> bool {
        other.width > self.width && other.height > self.height
    }
}

/// Combines the maximum, minimum and preferred sizes for a cell.
#[derive(Clone)]
pub struct SizeGrouping {
    pub minimum:   Size,
    pub maximum:   Size,
    pub preferred: Size,
}

impl Default for SizeGrouping {
    fn default() -> Self {
        SizeGrouping{
            minimum:   Size{width: 0.0, height: 0.0},
            preferred: Size{width: 0.0, height: 0.0},
            maximum:   Size{width: f32::MAX, height: f32::MAX},
        }
    }
}

impl SizeGrouping {
    pub fn join(a: &SizeGrouping, b: &SizeGrouping) -> SizeGrouping {
        SizeGrouping{
            minimum:   Size::join_max(&a.minimum,   &b.minimum),
            preferred: Size::join_max(&a.preferred, &b.preferred),
            maximum:   Size::join_min(&a.maximum,   &b.maximum),
        }
    }

    pub fn spread(&self, divisions: f32) -> SizeGrouping {
        SizeGrouping{
            minimum:   self.minimum.spread(divisions),
            preferred: self.preferred.spread(divisions),
            maximum:   self.maximum.spread(divisions),
        }
    }

    pub fn padded(&self, padding: Rectangle) -> SizeGrouping {
        SizeGrouping{
            minimum:   self.minimum.padded(padding),
            preferred: self.preferred.padded(padding),
            maximum:   self.maximum.padded(padding),
        }
    }

    /// Attempts to fit an `item` of a given size within an `area`, subject
    /// to layout rules specified by `flags`. Returns the X, Y coordinates
    /// as well as width and height of the box fitted to the area.
    pub fn box_fit(&self, area: &Size, prop: &CellProperties) -> (f32, f32, f32, f32) {
        let pad_width  = prop.padding.left + prop.padding.right;
        let pad_height = prop.padding.top  + prop.padding.bottom;

        // combine maximum width and area width, depending on if fill has been activated
        let w = if prop.flags.contains(CellFlags::FillHorizontal) {
            f32::min(self.maximum.width  , area.width - pad_width)
        } else {
            f32::min(self.preferred.width, area.width - pad_width)
        };

        // combine maximum height and area height, depending on if fill has been activated
        let h = if prop.flags.contains(CellFlags::FillVertical) {
            f32::min(self.maximum.height  , area.height - pad_height)
        } else {
            f32::min(self.preferred.height, area.height - pad_height)
        };

        // find horizontal location of output box
        let x = if prop.flags.contains(CellFlags::AnchorRight) {
            // take size of the area and remove width, will anchor us to the right side
            area.width - prop.padding.right - w
        } else if prop.flags.contains(CellFlags::AnchorHorizontalCenter) {
            // tricky because we have to find the midpoint, then adjust by half of width
            // XXX this ought to still work, because the padding is on the "outside" of our center
            (area.width / 2.0) - (w / 2.0)
        } else {
            // AnchorLeft is the same as doing nothing, so we just put this on the left side.
            prop.padding.left
        };

        // find vertical location of output box
        let y = if prop.flags.contains(CellFlags::AnchorBottom) {
            // take size of the area and remove height, will anchor us to the top side
            area.height - prop.padding.bottom - h
        } else if prop.flags.contains(CellFlags::AnchorHorizontalCenter) {
            // tricky because we have to find the midpoint, then adjust by half of height
            // XXX this ought to still work, because the padding is on the "outside" of our center
            (area.height / 2.0) - (h / 2.0)
        } else {
            // AnchorTop is the same as doing nothing, so we just put this on the top side.
            prop.padding.top
        };

        (x, y, w, h)
    }
}

bitflags! {
    pub struct CellFlags: u16 {
        const None                   = 0b0000_0000_0000_0000;
        /// Expands cell to fill all remaining horizontal space.
        const ExpandHorizontal       = 0b0000_0000_0000_0001;
        /// Expands cell to fill all remaining vertical space.
        const ExpandVertical         = 0b0000_0000_0000_0010;
        /// Expands cell's contents to fill all remaining horizontal space.
        const FillHorizontal         = 0b0000_0000_0000_0100;
        /// Expands cell's contents to fill all remaining vertical space.
        const FillVertical           = 0b0000_0000_0000_1000;
        /// Anchors the cell to the top of its available space.
        const AnchorTop              = 0b0000_0000_0001_0000;
        /// Anchors the cell to the bottom of its available space.
        const AnchorBottom           = 0b0000_0000_0010_0000;
        /// Anchors the cell to the left of its available space.
        const AnchorLeft             = 0b0000_0000_0100_0000;
        /// Anchors the cell to the right of its available space.
        const AnchorRight            = 0b0000_0000_1000_0000;
        /// Anchors the cell to the center of its available space, horizontally.
        const AnchorHorizontalCenter = 0b0000_0001_0000_0000;
        /// Anchors the cell to the center of its available space, vertically.
        const AnchorVerticalCenter   = 0b0000_0010_0000_0000;
        /// Cell will be the same size as all cells which are uniform.
        const Uniform                = 0b0000_0100_0000_0000;
    }
}

/// Allows a closure to ensure a layout item has been placed where the
/// layout engine decided it should go. The parameters are the `x`,
/// `y` coordinates, and the `width`/`height` respectively.
pub type PositioningFn = FnMut(f32, f32, f32, f32);

/// Encapsulates all properties for a cell; contributes to eventual layout decisions.
pub struct CellProperties {
    /// Controls the desired sizes for this cell.
    pub size:     SizeGrouping,
    /// Controls various binary flags for the cell.
    pub flags:    CellFlags,
    /// Controls how many columns this cell will occupy.
    pub colspan:  u8,
    /// Controls how many pixels are intentionally wasted around this cell.
    pub padding:  Rectangle,
    /// Applies positioning updates for this cell. Note that this
    /// value always becomes `None` when cloned, so you cannot set
    /// default callbacks for cell policies.
    pub callback: Option<Box<PositioningFn>>
}

impl Default for CellProperties {
    fn default() -> Self {
        CellProperties{
            size:     Default::default(),
            flags:    CellFlags::None,
            padding:  Default::default(),
            colspan:  1,
            callback: None,
        }
    }
}

impl Clone for CellProperties {
    fn clone(&self) -> Self {
        CellProperties{
            size:     self.size.clone(),
            flags:    self.flags,
            padding:  self.padding,
            colspan:  self.colspan,
            callback: None,
        }
    }
}

pub enum LayoutOp {
    /// Inserts a cell in the resulting layout.
    Cell(CellProperties),
    /// Inserts a row break in the resulting layout.
    Row,
}

#[derive(Default)]
pub struct TableLayout {
    pub cell_defaults:   CellProperties,
    pub row_defaults:    BTreeMap<u8, CellProperties>,
    pub column_defaults: BTreeMap<u8, CellProperties>,
    pub opcodes:         Vec<LayoutOp>,

    pub row: u8,
    pub column: u8,
}

impl CellProperties {
    pub fn new() -> Self {
        Default::default()
    }

    /// Inherits the default settings as determined by a
    /// `TableLayout`. Will first try to match the defaults for the
    /// column this would be added to, then the row, then the fallback
    /// defaults. Note that these defaults apply only if the cell
    /// was added next and if the defaults have not been changed
    /// since. The correct use of `with_defaults` is to initialize
    /// `CellProperties` for immediate insertion to a layout.
    pub fn with_defaults(layout: &TableLayout) -> Self {
        // try to get the column default
        let column_value = layout.column_defaults.get(&layout.column);
        if column_value.is_some() {
            return (*column_value.unwrap()).clone();
        }

        // try to get the row default
        let row_value = layout.row_defaults.get(&layout.row);
        if row_value.is_some() {
            return (*row_value.unwrap()).clone();
        }

        // just get the default i guess
        CellProperties{..layout.cell_defaults.clone()}
    }

    pub fn minimum_size(mut self, minimum: Size) -> Self {
        self.size.minimum = minimum;
        self
    }

    pub fn maximum_size(mut self, maximum: Size) -> Self {
        self.size.maximum = maximum;
        self
    }

    pub fn preferred_size(mut self, preferred: Size) -> Self {
        self.size.preferred = preferred;
        self
    }

    pub fn expand(mut self) -> Self {
        self.flags |= CellFlags::ExpandHorizontal | CellFlags::ExpandVertical;
        self
    }

    pub fn expand_horizontal(mut self) -> Self {
        self.flags |= CellFlags::ExpandHorizontal;
        self
    }

    pub fn expand_vertical(mut self) -> Self {
        self.flags |= CellFlags::ExpandVertical;
        self
    }

    pub fn fill(mut self) -> Self {
        self.flags |= CellFlags::FillHorizontal | CellFlags::FillVertical;
        self
    }

    pub fn fill_horizontal(mut self) -> Self {
        self.flags |= CellFlags::FillHorizontal;
        self
    }

    pub fn fill_vertical(mut self) -> Self {
        self.flags |= CellFlags::FillVertical;
        self
    }

    pub fn anchor_top(mut self) -> Self {
        self.flags |= CellFlags::AnchorTop;
        self
    }

    pub fn anchor_bottom(mut self) -> Self {
        self.flags |= CellFlags::AnchorBottom;
        self
    }

    pub fn anchor_left(mut self) -> Self {
        self.flags |= CellFlags::AnchorLeft;
        self
    }

    pub fn anchor_right(mut self) -> Self {
        self.flags |= CellFlags::AnchorRight;
        self
    }

    pub fn anchor_center(mut self) -> Self {
        self.flags |= CellFlags::AnchorHorizontalCenter | CellFlags::AnchorVerticalCenter;
        self
    }

    pub fn anchor_horizontal_center(mut self) -> Self {
        self.flags |= CellFlags::AnchorHorizontalCenter;
        self
    }

    pub fn anchor_vertical_center(mut self) -> Self {
        self.flags |= CellFlags::AnchorVerticalCenter;
        self
    }

    pub fn uniform(mut self) -> Self {
        self.flags |= CellFlags::Uniform;
        self
    }

    pub fn colspan(mut self, span: u8) -> Self {
        self.colspan = span;
        self
    }

    pub fn callback(mut self, fun: Box<PositioningFn>) -> Self {
        self.callback = Option::Some(fun);
        self
    }

    /// Sets the padding around this cell to the supplied top, left, right and bottom values as
    /// specified by a rectangle struct.
    pub fn padding(mut self, pad: &Rectangle) -> Self {
        self.padding = *pad;
        self
    }

    pub fn padding_all(mut self, pad: f32) -> Self {
        self.padding.top    = pad;
        self.padding.left   = pad;
        self.padding.bottom = pad;
        self.padding.right  = pad;
        self
    }

    /// Sets the padding on the top side of this cell.
    pub fn padding_top(mut self, pad: f32) -> Self {
        self.padding.top = pad;
        self
    }

    /// Sets the padding on the left side of this cell.
    pub fn padding_left(mut self, pad: f32) -> Self {
        self.padding.left = pad;
        self
    }

    /// Sets the padding on the bottom side of this cell.
    pub fn padding_bottom(mut self, pad: f32) -> Self {
        self.padding.bottom = pad;
        self
    }

    /// Sets the padding on the right side of this cell.
    pub fn padding_right(mut self, pad: f32) -> Self {
        self.padding.right = pad;
        self
    }
}

impl TableLayout {
    pub fn new() -> TableLayout {
        Default::default()
    }

    /// Calculates the number of rows and columns which exist in this table layout.
    pub fn get_rows_cols(&self) -> (u8, u8) {
        let mut cols   = 0;
        let mut colcur = 0;
        let mut rows   = 0;

        for op in &self.opcodes {
            match op {
                LayoutOp::Cell(cp) => { colcur += cp.colspan },
                LayoutOp::Row      => { cols = max(cols, colcur); colcur = 0; rows += 1 },
            }
        }

        if colcur > 0 {
            cols = max(cols, colcur);
            rows += 1;
        }

        (rows, cols)
    }

    /// Removes all layout declarations from the table. Does not remove row or column defaults.
    pub fn clear(&mut self) {
        self.row = 0;
        self.column = 0;
        self.opcodes.clear()
    }

    /// Removes all layout declarations and resets ALL settings to factory default.
    pub fn full_clear(&mut self) {
        self.clear();
        self.row_defaults.clear();
        self.column_defaults.clear();
        self.cell_defaults = Default::default()
    }

    /// Adds a new row to the layout.
    pub fn with_row(&mut self) -> &mut Self {
        self.opcodes.push(LayoutOp::Row);
        self.row += 1;
        self.column = 0;
        self
    }

    /// Hands the cell off to the layout.
    pub fn with_cell(&mut self, properties: CellProperties) -> &mut Self {
        self.column += properties.colspan;
        self.opcodes.push(LayoutOp::Cell(properties));
        self
    }

    pub fn impose(&mut self, width: f32, height: f32) {
        let mut row: u8 = 0;
        let mut col: u8 = 0;

        let (total_rows, total_cols) = self.get_rows_cols();
        if total_cols == 0 {return} // short-circuiting opportunity

        let mut col_sizes: Vec<SizeGrouping> = Vec::with_capacity(total_cols as usize);
        // XXX resize_with is unstable, but would do what we want just fine
        for _i in 0..total_cols {
            col_sizes.push(Default::default());
        }

        // XXX resize_with is unstable, but would do what we want just fine
        let mut row_sizes: Vec<SizeGrouping> = Vec::with_capacity(total_cols as usize);
        for _i in 0..total_rows {
            row_sizes.push(Default::default());
        }

        let mut has_xexpand: Vec<bool> = Vec::with_capacity(total_cols as usize);
        for _i in 0..total_cols {
            has_xexpand.push(false);
        }

        let mut has_yexpand: Vec<bool> = Vec::with_capacity(total_rows as usize);
        for _i in 0..total_rows {
            has_yexpand.push(false);
        }

        // We determine size preferences for each column in the layout.
        for op in &self.opcodes {
            match op {
                LayoutOp::Cell(cp) => {
                    match cp.colspan {
                        // If a cell has a span of zero, that is kind of stupid and it basically doesn't exist.
                        0 => {},
                        _ => {
                            let midget = cp.size.padded(cp.padding).spread(f32::from(cp.colspan));
                            row_sizes[row as usize] =
                                SizeGrouping::join(&row_sizes[row as usize], &cp.size);
                            if cp.flags.contains(CellFlags::ExpandVertical) {
                                has_yexpand[row as usize] = true
                            }
                            for _i in 0..cp.colspan {
                                if cp.flags.contains(CellFlags::ExpandHorizontal) {
                                    has_xexpand[col as usize] = true
                                }
                                col_sizes[col as usize] = SizeGrouping::join(&col_sizes[col as usize], &midget);
                                col += 1;
                            }
                        }
                    }
                }
                // flop to a new row
                LayoutOp::Row => {
                    row += 1;
                    col = 0;
                }
            }
        }

        let mut slack: Vec<f32> = Vec::new();

        // Calculate error along width distribution
        let mut error = width;
        for c in &col_sizes {
            // Error is what remains once we have given each column its preferred size.
            error -= c.preferred.width;
        }

        if error > 0.0 { // Extra space; relax the layout if we need to
            // Figure out how many columns are expanding horizontally.
            let expansions = has_xexpand.iter().filter(|x| **x).count();
            if expansions > 0 {
                let amount = error / expansions as f32;
                for (i, e) in has_xexpand.iter().enumerate() {
                    if *e {
                        col_sizes[i].preferred.width += amount;
                    }
                }
            }
        } else if error < 0.0 { // Not enough space; tense up some more!
            let error = -error;
            // We need to find slack space for each column
            let mut total_slack: f32 = 0.0;
            slack.clear();
            slack.resize(total_cols as usize, 0.0);
            for (i, x) in col_sizes.iter().map(|x| x.preferred.width - x.minimum.width).enumerate() {
                slack[i] = x;
                total_slack += x;
            }

            // XXX if error > total_slack, it is impossible to solve this constraint
            // spread error across slack space, proportionate to this areas slack participation
            for mut s in &mut slack {
                let norm = *s / total_slack;
                let error_over_slack = error * norm;
                *s -= error_over_slack
            }

            // Spread error across slack space.
            for (i, x) in slack.iter().enumerate() {
                col_sizes[i].preferred.width =
                    f32::max(col_sizes[i].minimum.width + *x, 0.0);
            }
        }

    	// Calculate error along height distribution
    	let mut error = height;
    	for c in &row_sizes {
                // Error is what remains once we have given each row its preferred size.
                error -= c.preferred.height;
    	}

        if error > 0.0 { // Extra space; relax the layout if we need to
            // Figure out how many columns are expanding horizontally.
            let expansions = has_yexpand.iter().filter(|y| **y).count();
            if expansions > 0 {
                let amount = error / expansions as f32;
                for (i, e) in has_yexpand.iter().enumerate() {
                    if *e {
                        row_sizes[i].preferred.height += amount;
                    }
                }
            }
        } else if error < 0.0 { // Not enough space; tense up some more!
            let error = -error;
            // We need to find slack space for each row
            let mut total_slack: f32 = 0.0;
            slack.clear();
            slack.resize(total_rows as usize, 0.0);
            for (i, y) in row_sizes.iter().map(|y| y.preferred.height - y.minimum.height).enumerate() {
                slack[i] = y;
                total_slack += y;
            }

            // XXX if error > total_slack, it is impossible to solve this constraint
            // spread error across slack space, proportionate to this areas slack participation
            for mut s in &mut slack {
                let norm = *s / total_slack;
                let error_over_slack = error * norm;
                *s -= error_over_slack
            }

            // Spread error across slack space.
            for (i, y) in slack.iter().enumerate() {
                row_sizes[i].preferred.height =
                    f32::max(row_sizes[i].minimum.height + *y, 0.0);
            }
        }

        // Preparations complete. Now we pass the news along to our client.
        let mut x = 0.0;
        let mut y = 0.0;
        row = 0;
        col = 0;
        for mut op in &mut self.opcodes {
            // NB can probably make this mutable, and update it only when the row changes
            let height = row_sizes[row as usize].preferred.height;
            match op {
                // Something that needs to be placed.
                LayoutOp::Cell(cp) => match &cp.colspan {
                    0 => {}, // Ignore this cell.
                    _ => {
                        let mut width: f32 = 0.0;
                        for _i in 0..cp.colspan {
                            width += col_sizes[col as usize].preferred.width;
                            col += 1;
                        }
                        let s = Size{width, height};
                        let (bx, by, bw, bh) = cp.size.box_fit(&s, &cp);

                        // Run callback to impose layout.
                        match &mut cp.callback {
                            Some(cb) => {
                                (*cb)(x+bx, y+by, bw, bh);
                            }
                            None => {},
                        }

                        x += width;
                    }
                },
                // Increment to next row; reset placement cursors.
                LayoutOp::Row => {
                    x = 0.0;
                    y += height;
                    row += 1;
                    col = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod test;
