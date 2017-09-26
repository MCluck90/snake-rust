use ggez::graphics::{Rect};
use std::time::Duration;

pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect {
        x: x + (w / 2.0),
        y: y + (h / 2.0),
        w: w,
        h: h
    }
}

pub fn to_ms(dt: Duration) -> u64 {
    let nanos = dt.subsec_nanos() as u64;
    (1000*1000*1000 * dt.as_secs() + nanos)/(1000 * 1000)
}