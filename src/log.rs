use colorful::{Color, Colorful};
pub fn success(message: &str) {
    println!("{}", message.color(Color::Green4).bold());
}

pub fn error(message: &str) {
    println!(
        "{}",
        message.color(Color::Blue).bg_color(Color::Yellow).bold()
    );
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// pub fn info(message: &str) {
//     println!(
//         "{}",
//         message.color(Color::Blue).bg_color(Color::Yellow).bold()
//     );
// }
//
//
