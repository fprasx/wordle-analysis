/**
Take in a float, and based on how small/large it is
assign it a color, higher means greener
 */
pub fn add_color<'a>(num: f32) -> String {
    if num < 0.03 {
        format!("\x1B[38;2;150;150;250m{:.2}\x1B[0m", num)
    } else if num < 0.06 {
        format!("\x1B[38;2;0;170;0m{:.2}\x1B[0m", num)
    } else if num < 0.09 {
        format!("\x1B[38;2;0;190;0m{:.2}\x1B[0m", num)
    } else if num < 0.12 {
        format!("\x1B[38;2;0;210;0m{:.2}\x1B[0m", num)
    } else {
        format!("\x1B[38;2;0;255;0m{:.2}\x1B[0m", num)
    }
}
