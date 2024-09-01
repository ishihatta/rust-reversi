use crate::banmen::Banmen;
use crate::banmen::BANMEN_SIZE;

const MAX_DEPTH: usize = 8;

fn calc_banmen_score(banmen: &Banmen, my_ban: u8, next_ban: u8, is_prev_pass: bool, depth: usize) -> f32 {
    if banmen.remain <= 0 || depth >= MAX_DEPTH {
        let win = banmen.which_is_win();
        return if win == my_ban {
            1f32
        } else {
            0f32
        };
    }
    let mut score = 0f32;
    let mut plus_num = 0;
    let mut placed = false;
    for y in 0..BANMEN_SIZE {
        for x in 0..BANMEN_SIZE {
            let next_banmen_opt = banmen.put_piece(x, y, next_ban);
            if let Some(next_banmen) = next_banmen_opt {
                let s = calc_banmen_score(&next_banmen, my_ban,  3 - next_ban, false, depth + 1);
                if next_ban == my_ban {
                    if s == 1f32 {
                        return 1f32;
                    }
                    if s > score {
                        score = s;
                    }
                } else {
                    score += s;
                    plus_num += 1;
                }
                placed = true;
            }
        }
    }
    if plus_num > 0 {
        score /= plus_num as f32;
    }
    // もし打つ場所がなかったら
    if !placed {
        if is_prev_pass {
            // 前回もパスだった場合（＝敵味方ともに置けない状態）はここでゲーム終了
            let win = banmen.which_is_win();
            return if win == my_ban {
                1f32
            } else {
                0f32
            };
        } else {
            score = calc_banmen_score(banmen, my_ban, 3 - next_ban, true, depth + 1);
        }
    }
    score
}

pub fn calc_next_place(banmen: &Banmen, ban: u8) -> Option<(usize, usize)> {
    let mut max_score = -1f32;
    let mut px = 0;
    let mut py = 0;
    'looptop: for y in 0..BANMEN_SIZE {
        for x in 0..BANMEN_SIZE {
            let next_banmen_opt = banmen.put_piece(x, y, ban);
            if let Some(next_banmen) = next_banmen_opt {
                let score = calc_banmen_score(&next_banmen, ban,  3 - ban, false, 1);
                if score > max_score {
                    max_score = score;
                    px = x;
                    py = y;
                    if max_score >= 1f32 {
                        break 'looptop;
                    }
                }
            }
        }
    }
    if max_score >= 0f32 {
        // let pl = if ban == 1 { "O" } else { "X" };
        // println!("{} score={}", pl, max_score);
        Some((px, py))
    } else {
        None
    }
}
