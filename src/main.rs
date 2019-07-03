use std::cell::Cell;
use std::cell::RefCell;

struct ZigZag {
    rows: usize,
    cols: usize,
    value: Cell<u32>,
    array: RefCell<Vec<Vec<u32>>>,
}

impl ZigZag {
    fn new(rows: usize, cols: usize) -> ZigZag {
        let array = RefCell::new(vec![vec![0u32; cols]; rows]);
        ZigZag {
            rows,
            cols,
            value: Cell::new(0),
            array,
        }
    }
    fn can_move_ne(&self, row: usize, col: usize) -> bool {
        if row <= 0 || col >= self.cols - 1 {
            false
        } else {
            true
        }
    }

    fn can_move_s(&self, row: usize) -> bool {
        if row >= self.rows - 1 {
            false
        } else {
            true
        }
    }

    fn can_move_e(&self, col: usize) -> bool {
        if col >= self.cols - 1 {
            return false;
        } else {
            true
        }
    }

    fn can_move_sw(&self, row: usize, col: usize) -> bool {
        if row >= self.rows - 1 || col <= 0 {
            false
        } else {
            true
        }
    }

    fn move_e(col: &mut usize) {
        *col += 1;
    }

    fn move_s(row: &mut usize) {
        *row += 1;
    }

    fn move_ne(row: &mut usize, col: &mut usize) {
        *row -= 1;
        *col += 1;
    }

    fn move_sw(row: &mut usize, col: &mut usize) {
        *row += 1;
        *col -= 1;
    }


    fn done(&self, row: usize, col: usize) -> bool {
        if row == self.cols - 1 && col == self.cols - 1 {
            true
        } else {
            false
        }
    }

    fn fill_value(&self, row: usize, col: usize) {
        let mut val = self.value.get();
        val += 1;
        self.array.borrow_mut()[row][col] = val;
        self.value.set(val);
    }

    fn zig_zag(&self) {
        let mut row = 0;
        let mut col = 0;
        while !self.done(row, col) {
            if !self.can_move_ne(row, col) && self.can_move_e(col) {
                ZigZag::move_e(&mut col);
                self.fill_value(row, col);
                while self.can_move_sw(row, col) {
                    ZigZag::move_sw(&mut row, &mut col);
                    self.fill_value(row, col);
                }
            }

            if !self.can_move_sw(row, col) && self.can_move_s(row)
                && self.can_move_ne(row, col) && self.can_move_e( col) {
                ZigZag::move_s(&mut row);
                self.fill_value(row, col);
                while self.can_move_ne(row, col) {
                    ZigZag::move_ne(&mut row, &mut col);
                    self.fill_value(row, col);
                }
            }

            if self.can_move_sw(row, col) && self.can_move_s(row)
                && !self.can_move_ne(row, col) && !self.can_move_e(col) {
                ZigZag::move_e(&mut col);
                while self.can_move_sw(row, col) {
                    ZigZag::move_sw(&mut row, &mut col);
                    self.fill_value(row, col);
                }
            }

            if !self.can_move_sw(row, col) && !self.can_move_s(row)
                && self.can_move_ne(row, col) && self.can_move_e( col) {
                ZigZag::move_e( &mut col);
                self.fill_value(row, col);
                while self.can_move_ne(row, col) {
                    ZigZag::move_ne(&mut row, &mut col);
                    self.fill_value(row, col);
                }
            }
        }
    }
    fn print_zig_zag(&self) {
        for i in self.array.borrow().iter() {
            for j in i {
                print!("{:>5}", j);
            }
            println!();
        }
    }
}

fn main() {
    let zig_zag = ZigZag::new(10, 10);
    zig_zag.zig_zag();
    zig_zag.print_zig_zag();
}
