use rand::random;

use crate::{Pixel, RAMP, RAMP_LEN};

pub fn wave(uv: Pixel, time: f64, fb: &mut String) {
    // Procedural shader function (trigonometric pattern)
    let wave = (uv.x * 10.0 + time).cos() * (uv.y * 10.0 + time).sin();

    // Map [-1.0, 1.0] range to [0.0, 1.0] intensity
    let intensity = (wave + 1.0) / 2.0;

    // Clamp index safely inside our ASCII ramp length
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn tan_wave(uv: Pixel, time: f64, fb: &mut String) {
    let mut waves = vec![];

    let zoom = time.sin() + time.cos() * 8.; // Increase this value to zoom out more, decrease to zoom in
    for i in 0..8 {
        let wave = (uv.x * zoom * random::<f64>().sin() + time).sin()
            + (uv.y * zoom * random::<f64>().sin() + time).sin();
        waves.push(wave);
    }

    let intensity_avg: f64 = waves.iter().sum::<f64>() / waves.len() as f64;

    // Map [-1.0, 1.0] range to [0.0, 1.0] intensity
    let intensity = (intensity_avg as f64 + 1.0) / 2.0;

    // Clamp index safely inside our ASCII ramp length
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn bumps(uv: Pixel, time: f64, fb: &mut String) {
    let mut val = uv.x + uv.y;

    if uv.x != 0.5 && uv.y != 0.5 {
        val = 1.0
    } 

    if uv.x != 0.75 && uv.y != 0.75 {
        val = 1.0
    } 

    // Map [-1.0, 1.0] range to [0.0, 1.0] intensity
    let intensity = (val + 1.0) / 2.0;

    // Clamp index safely inside our ASCII ramp length
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn plasma(uv: Pixel, time: f64, fb: &mut String) {
    let v1 = (uv.x * 10.0 + time).sin();
    let v2 = (uv.y * 10.0 + time).sin();
    let v3 = ((uv.x * 10.0 + uv.y * 10.0) + time).sin();
    let cx = uv.x * 10.0 + time.sin();
    let cy = uv.y * 10.0 + time.cos();
    let v4 = (cx * cx + cy * cy).sqrt().sin();
    
    let v = v1 + v2 + v3 + v4;
    let intensity = (v.sin() + 1.0) / 2.0;
    
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn spiral(uv: Pixel, time: f64, fb: &mut String) {
    let dx = uv.x - 0.5;
    let dy = uv.y - 0.5;
    let radius = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx);
    
    let v = (angle * 5.0 + radius * 20.0 - time * 5.0).sin();
    let intensity = (v + 1.0) / 2.0;
    
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn ripples(uv: Pixel, time: f64, fb: &mut String) {
    let dx = uv.x - 0.5;
    let dy = uv.y - 0.5;
    let dist = (dx * dx + dy * dy).sqrt();
    
    let v = (dist * 50.0 - time * 10.0).sin();
    // damp out with distance
    let v = v * (1.0 - dist * 2.0).max(0.0);
    
    let intensity = (v + 1.0) / 2.0;
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}

pub fn interference(uv: Pixel, time: f64, fb: &mut String) {
    let dx1 = uv.x - 0.3;
    let dy1 = uv.y - 0.3;
    let dx2 = uv.x - 0.7;
    let dy2 = uv.y - 0.7;
    
    let d1 = (dx1 * dx1 + dy1 * dy1).sqrt();
    let d2 = (dx2 * dx2 + dy2 * dy2).sqrt();
    
    let v1 = (d1 * 40.0 - time * 5.0).sin();
    let v2 = (d2 * 40.0 - time * 5.0).sin();
    
    let intensity = ((v1 + v2) / 2.0 + 1.0) / 2.0;
    
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}
