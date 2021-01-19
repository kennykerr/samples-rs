fn main() {
    windows::build!(
        windows::win32::base::{HWND, MB_OK}
        windows::win32::menu_rc::MessageBoxA
    );
}
