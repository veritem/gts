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

// pub fn info(message: &str) {
//     println!(
//         "{}",
//         message.color(Color::Blue).bg_color(Color::Yellow).bold()
//     );
// }
