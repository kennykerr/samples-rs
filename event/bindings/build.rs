fn main() {
    windows::build!(
        windows::win32::backup::{CreateEventW, SetEvent, WaitForSingleObject}
        windows::win32::win_prog::CloseHandle
    );
}
