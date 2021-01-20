use bindings::{
    windows::win32::backup::{CreateEventW, SetEvent, WaitForSingleObject},
    windows::win32::win_prog::CloseHandle,
};

fn main() -> windows::Result<()> {
    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true.into(),
            false.into(),
            std::ptr::null_mut(),
        );

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == 0); // https://github.com/microsoft/win32metadata/issues/77

        CloseHandle(event).ok()
    }
}
