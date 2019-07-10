extern crate bitflags;
extern crate libc;

extern crate gdk;
extern crate glib;
extern crate glib_sys;
extern crate gtk;
extern crate gtk_sys;
extern crate gtk_layer_shell_sys;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use auto::functions::*;
pub use auto::*;

