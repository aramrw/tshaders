<img width="auto" height="330" alt="tshaders" src="https://github.com/user-attachments/assets/80d0fe65-1314-4101-a760-a2dfe114d9a6" />

```rust
pub fn wave(uv: Pixel, time: f64, fb: &mut String) {
    // Procedural shader function (trigonometric pattern)
    let wave = (uv.x * 10.0 + time).cos() * (uv.y * 10.0 + time).sin();

    // Map [-1.0, 1.0] range to [0.0, 1.0] intensity
    let intensity = (wave + 1.0) / 2.0;

    // Clamp index safely inside our ASCII ramp length
    let char_idx = ((intensity * (RAMP_LEN - 1.0)) as usize).min(RAMP.len() - 1);
    fb.push(RAMP[char_idx] as char);
}
```
