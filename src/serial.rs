
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!{
	// Uart interface. We use a mutex for safety.
	pub static ref SERIAL1: Mutex<SerialPort> = {
		// 0x3f8 is the standard address for first interface
		let mut port = unsafe { SerialPort::new(0x3F8) };
		port.init();
		Mutex::new(port)
	};
}

// a function to print into SERIAL1
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use core::fmt::Write;
	SERIAL1
		.lock()  // to be able to use the mutex wrapped object
		.write_fmt(args)  // SerialPort provides self.write_fmt(args) ie implements fmt::Write
		.expect("Serial Printing Failed");  // on error
}

#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
