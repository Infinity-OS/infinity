use vga_buffer;
use vga_buffer::ColorCode as ColorCode;
use vga_buffer::Color as Color;

const DEFAULT_COLOR_CODE: ColorCode = vga_buffer::DEFAULT_COLOR_CODE;
// const DEFAULT_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Black);
const SUCCESS_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Green);
const ERROR_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Red);
const WARNING_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Brown);
const INFO_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Blue);

pub enum MessageType {
    SUCCESS,
    ERROR,
    WARNING,
    INFO,
    DEFAULT
}

fn restore_defauls() {
    vga_buffer::WRITER.lock().set_color_code(DEFAULT_COLOR_CODE);
}


pub fn kprint(message_type: MessageType, message: &str) {
    match message_type {
        MessageType::SUCCESS => {
            vga_buffer::WRITER.lock().set_color_code(SUCCESS_COLOR_CODE);
        }
        MessageType::ERROR => {
            vga_buffer::WRITER.lock().set_color_code(ERROR_COLOR_CODE);
        }
        MessageType::WARNING => {
            vga_buffer::WRITER.lock().set_color_code(WARNING_COLOR_CODE);
        }
        MessageType::INFO => {
            vga_buffer::WRITER.lock().set_color_code(INFO_COLOR_CODE);
        }
        MessageType::DEFAULT => {
            vga_buffer::WRITER.lock().set_color_code(DEFAULT_COLOR_CODE);
        }
        _ => {
            panic!("That message type doesn't exist.")
        }
    }
    println!("{}", message);
    restore_defauls();
}

