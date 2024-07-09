use log::Level;

pub fn level(level: Level) -> String {
    match level {
        Level::Error => "\x1b[1;101mE\x1b[0m",
        Level::Warn => "\x1b[1;103mW\x1b[0m",
        Level::Info => "\x1b[1;102mI\x1b[0m",
        Level::Debug => "\x1b[1;104mD\x1b[0m",
        Level::Trace => "\x1b[1;105mT\x1b[0m",
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

fn string_to_color(s: &str) -> (u8, u8, u8) {
    hsl_to_rgb((hash_string(s) as f64).rem_euclid(360.0), 0.9f64, 0.70f64)
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

fn mix_colors(color1: (u8, u8, u8) , color2: (u8, u8, u8) , ratio: f32) -> (u8, u8, u8)  {
    let r = (color1.0 as f32 * (1.0 - ratio) + color2.0 as f32 * ratio).round() as u8;
    let g = (color1.1 as f32 * (1.0 - ratio) + color2.1 as f32 * ratio).round() as u8;
    let b = (color1.2 as f32 * (1.0 - ratio) + color2.2 as f32 * ratio).round() as u8;

    (r, g, b)
}

pub fn module(module: &str) -> String {
    let modules: Vec<&str> = module.split("::").collect();
    let mut result = String::new();
    let mut last_color: Option<(u8, u8, u8)> = None;
    for m in modules {
        let mut color = string_to_color(m);
        if let Some(color2) = last_color{
            color = mix_colors(color, color2, 0.5);
        }
        last_color = Some(color);
        result.push_str(&format!(
            "\x1b[1;38;2;{};{};{}m{}\x1b[0m",
            color.0, color.1, color.2, m
        ));
        result.push_str("\x1b[1;97m::\x1b[0m");
    }
    result
}
