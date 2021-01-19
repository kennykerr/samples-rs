fn main() {
    windows::build!(
        windows::win32::base::{BOOL, HWND, LPARAM}
        windows::win32::menu_rc::{EnumWindows, GetWindowTextW}
    );
}
