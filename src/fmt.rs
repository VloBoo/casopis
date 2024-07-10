use log::Level;

pub fn level(level: Level) -> String {
    match level {
        Level::Error => "\x1b[31mERROR",
        Level::Warn => "\x1b[33mWARN ",
        Level::Info => "\x1b[32mINFO ",
        Level::Debug => "\x1b[34mDEBUG",
        Level::Trace => "\x1b[35mTRACE",
    }
    .to_owned()
}

fn hash_string(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in s.bytes() {
        hash = (hash.wrapping_shl(5))
            .wrapping_add(hash)
            .wrapping_add(u32::from(byte));
    }
    hash
}

fn string_to_h(s: &str) -> f64 {
    (hash_string(s) as f64).rem_euclid(360.0)
}

fn h_to_rgb(h: f64) -> (u8, u8, u8) {
    hsl_to_rgb(h, 0.9f64, 0.70f64)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = match h {
        0.0..=60.0 => (c, x, 0.0),
        60.0..=120.0 => (x, c, 0.0),
        120.0..=180.0 => (0.0, c, x),
        180.0..=240.0 => (0.0, x, c),
        240.0..=300.0 => (x, 0.0, c),
        300.0..=360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}

fn mix_colors(color1: (u8, u8, u8), color2: (u8, u8, u8), ratio: f32) -> (u8, u8, u8) {
    let r = (color1.0 as f32 * (1.0 - ratio) + color2.0 as f32 * ratio).round() as u8;
    let g = (color1.1 as f32 * (1.0 - ratio) + color2.1 as f32 * ratio).round() as u8;
    let b = (color1.2 as f32 * (1.0 - ratio) + color2.2 as f32 * ratio).round() as u8;

    (r, g, b)
}

fn mix_h(h: f64, h2: f64, ratio: f64) -> f64 {
    h * (1.0 - ratio) + h2 * ratio
}

pub fn module(module: &str) -> String {
    let modules: Vec<&str> = module.split("::").collect();
    let mut result = String::new();
    let mut last_h: Option<f64> = None;
    for m in modules {
        let mut h = string_to_h(m);
        if let Some(h2) = last_h {
            h = mix_h(h, h2, 0.5);
        }
        last_h = Some(h);
        let color = h_to_rgb(h);
        result.push_str(&format!(
            "\x1b[1;38;2;{};{};{}m{}",
            color.0, color.1, color.2, m
        ));
        result.push_str("\x1b[97m::");
    }
    result
}
