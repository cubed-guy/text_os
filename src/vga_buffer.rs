use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode(((background as u8) << 4)| (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH:  usize = 80;

#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code: color_code,
				});
				self.column_position += 1;
			}
		}
	}

	fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row-1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	pub fn write_string(&mut self, string: &str) {
		for byte in string.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
			// color_code: ColorCode(Color::Black as u8),
		};

		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

use spin::Mutex;
lazy_static::lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::LightRed, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
	});
}

// pub fn yet_another_printer() {
	

// 	WRITER.write_byte(b'H');
// 	WRITER.write_string("ello, ");
// 	WRITER.write_string("Wörld!");

// 	use fmt::Write;

// 	writeln!(WRITER, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
// 	WRITER.write_string("Hello, World!");

// }

#[macro_export]
macro_rules! println {
	// no need to import print!() to use println!()
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	// if an interrupt occurs while the mutex is locked
	// a deadlock occurs.
	// Thus, we'll prevent the handling of interrupts while the mutex is locked.
	use x86_64::instructions::interrupts;
	interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	});
}


#[test_case]  // test cases pass if there is no panic
fn test_println_simple() {
	println!("Hello, World!");
}

#[test_case]
fn test_println_many() {
	for i in 1..200 {
		println!("REEEEEEEE {}", i);
	}
}

#[test_case]
fn test_println_output() {  // if the output shows up in the vga buffer
	let s = "This should fit in one line";

	use core::fmt::Write;
	use x86_64::instructions::interrupts;
	interrupts::without_interrupts ( || {
		let mut writer = WRITER.lock();  // global static buffer
		writeln!(writer, "\n{}", s).expect("could not write to vga buffer");
		for (i, c) in s.chars().enumerate() {
			let screen_char = writer
				.buffer
				// BUFFER_HEIGHT-1 is the last line. ln shifts it up one
				.chars[BUFFER_HEIGHT - 2][i]
				.read();
			assert_eq!(char::from(screen_char.ascii_character), c);
		}
	});
}
