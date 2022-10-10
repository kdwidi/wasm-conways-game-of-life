extern crate console_error_panic_hook;
extern crate web_sys;
use web_sys::console;

pub fn set_panic_hook(error_message: &str) {
    console::error_1(&format!("{}",error_message).into());

    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// pub struct Timer<'a> {
//   name: &'a str,
// }

// impl<'a> Timer<'a> {
//     pub fn new (name: &'a str) -> Timer<'a> {
//       console::time_with_label(name);
//       Timer { name }
//     }
// }

// impl<'a> Drop for Timer<'a> {
//   fn drop(&mut self) {
//       console::time_end_with_label(self.name);
//   }
// }

#[macro_export]
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!($( $t )* ).into());
  }
}