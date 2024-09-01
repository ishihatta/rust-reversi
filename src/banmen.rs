pub const BANMEN_SIZE: usize = 6;

pub struct Banmen {
    piece: [u8; BANMEN_SIZE * BANMEN_SIZE],
    pub remain: usize,
}

impl Banmen {
    pub fn new() -> Self {
        let mut p: [u8; BANMEN_SIZE * BANMEN_SIZE] = [0; BANMEN_SIZE * BANMEN_SIZE];
        p[Self::piece_index(BANMEN_SIZE / 2 - 1, BANMEN_SIZE / 2 - 1)] = 1;
        p[Self::piece_index(BANMEN_SIZE / 2, BANMEN_SIZE / 2)] = 1;
        p[Self::piece_index(BANMEN_SIZE / 2, BANMEN_SIZE / 2 - 1)] = 2;
        p[Self::piece_index(BANMEN_SIZE / 2 - 1, BANMEN_SIZE / 2)] = 2;
        Self {
            piece: p,
            remain: BANMEN_SIZE * BANMEN_SIZE - 4,
        }
    }

    fn piece_index(x: usize, y: usize) -> usize {
        y * BANMEN_SIZE + x
    }

    fn get_piece(&self, x: usize, y: usize) -> u8 {
        if x >= BANMEN_SIZE || y >= BANMEN_SIZE {
            return 0;
        }
        self.piece[Self::piece_index(x, y)]
    }

    pub fn put_piece(&self, x: usize, y: usize, piece: u8) -> Option<Banmen> {
        if x >= BANMEN_SIZE || y >= BANMEN_SIZE {
            return None;
        }
        if self.get_piece(x, y) != 0 {
            return None;
        }
        let mut new_piece = self.piece.clone();
        let mut placeable = false;
        for yy in -1isize..=1 {
            for xx in -1isize..=1 {
                if xx == 0 && yy == 0 { continue }
                let mut px = (x as isize) + xx;
                let mut py = (y as isize) + yy;
                if px < 0 || py < 0 || px >= BANMEN_SIZE as isize || py >= BANMEN_SIZE as isize { continue; }
                if self.get_piece(px as usize, py as usize) == 3 - piece {
                    let reverrceable = loop {
                        px += xx;
                        py += yy;
                        let p = self.get_piece(px as usize, py as usize);
                        if p == 0 {
                            break false;
                        } else if p == piece {
                            break true;
                        }
                    };
                    if reverrceable {
                        placeable = true;
                        let mut rx = (x as isize) + xx;
                        let mut ry = (y as isize) + yy;
                        while rx != px || ry != py {
                            new_piece[Self::piece_index(rx as usize, ry as usize)] = piece;
                            rx += xx;
                            ry += yy;
                        }
                    }
                }
            }
        }
        if placeable {
            new_piece[Self::piece_index(x, y)] = piece;
            Some(
                Banmen {
                    piece: new_piece,
                    remain: self.remain - 1,
                }
            )
        } else {
            None
        }
    }

    pub fn which_is_win(&self) -> u8 {
        let mut count1 = 0;
        let mut count2 = 0;
        for p in self.piece {
            if p == 1 {
                count1 += 1;
            } else if p == 2 {
                count2 += 1;
            }
        }
        return if count1 > count2 {
            1
        } else if count1 < count2 {
            2
        } else {
            0
        };
    }

    pub fn print(&self) {
        let mut num1 = 0;
        let mut num2 = 0;
        print!(" ");
        for i in 0..BANMEN_SIZE {
            print!("{}", i);
        }
        println!();
        for y in 0..BANMEN_SIZE {
            print!("{}", y);
            for x in 0..BANMEN_SIZE {
                match self.get_piece(x, y) {
                    1 => { print!("O"); num1 += 1 },
                    2 => { print!("X"); num2 += 1 },
                    _ => print!(" "),
                }
            }
            println!();
        }
        println!("[O: {}, X: {}]", num1, num2);
    }
}
