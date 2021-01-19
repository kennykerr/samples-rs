use bindings::{
    windows::win32::base::{HWND, MB_OK},
    windows::win32::menu_rc::MessageBoxA,
};

fn main() {
    unsafe {
        let caption = b"Hello\0";
        let text = b"World\0";

        MessageBoxA(
            HWND(0),
            text.as_ptr() as *const i8,
            caption.as_ptr() as *const i8,
            MB_OK as u32,
        );
    }
}
