const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
fn main() {
    let mut f: f64 = FREEZING_POINT_F;
    let c: f64 = fahrenheit_to_celsius(f);

    println!("{:.2}°F to {:.3}°C", FREEZING_POINT_F, c);

    for _ in 1..=5 {
        f = f + 1.0;
        println!("{:.2}°F to {:.3}°C", f, fahrenheit_to_celsius(f));
    }
}
