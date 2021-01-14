#![allow(unused)] // work around cargo bug
use crisscross::{Crossing, TilePosition};

fn round(n: f32, decimals: usize) -> f32 {
    let factor = 10_u64.pow(decimals as u32) as f64;
    ((n as f64 * factor).round() / factor) as f32
}

pub fn round_tp(tp: TilePosition) -> TilePosition {
    let TilePosition { x, y, rel_x, rel_y } = tp;
    TilePosition {
        x,
        y,
        rel_x: round(rel_x, 3),
        rel_y: round(rel_y, 3),
    }
}

pub fn round_cutoff(
    Crossing {
        valid: last_valid,
        invalid: first_invalid,
    }: Crossing,
) -> Crossing {
    Crossing {
        valid: last_valid.map(round_tp),
        invalid: first_invalid.map(round_tp),
    }
}
