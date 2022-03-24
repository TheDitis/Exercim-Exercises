const CPH_UNIT: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    cph(speed) * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

pub fn cph(speed: u8) -> f64 {
    speed as f64 * CPH_UNIT as f64
}

pub fn success_rate(speed: u8) -> f64 {
    match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}