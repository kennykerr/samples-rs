use bindings::{
    windows::win32::backup::{CreateEventW, SetEvent, WaitForSingleObject},
    windows::win32::win_prog::CloseHandle,
};

fn main() {
    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true.into(),
            false.into(),
            std::ptr::null_mut(),
        );

        assert!(event.0 != 0);

        let result = SetEvent(event);
        assert!(result.0 != 0);

        let result = WaitForSingleObject(event, 0);
        assert!(result == 0); // https://github.com/microsoft/win32metadata/issues/77

        let result = CloseHandle(event);
        assert!(result.0 != 0);
    }
}
