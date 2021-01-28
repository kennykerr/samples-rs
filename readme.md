This repo contains a few sample projects to get you started with [Rust for Windows](https://github.com/microsoft/windows-rs).

## clock

[clock](clock) - An example of using [Direct2D](https://docs.microsoft.com/en-us/windows/win32/direct2d/direct2d-overview) and various other Windows APIs.

## create_window

[create_window](create_window) - A simple example of creating a desktop window with [CreateWindowExA](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa) and friends.

## enum_windows

[enum_windows](enum_windows) - An example of using the [EnumWindows](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows) function.

## event

[event](event) - An example using the kernel [event object](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventa).

## message_box

[message_box](message_box) - An example using the age-old [MessageBox](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox) function.

## ocr

[ocr](ocr) - An example using the [Windows.Graphics.Imaging](https://docs.microsoft.com/en-us/uwp/api/Windows.Graphics.Imaging), [Windows.Media.Ocr](https://docs.microsoft.com/en-us/uwp/api/Windows.Media.Ocr), and [Windows.Storage](https://docs.microsoft.com/en-us/uwp/api/Windows.Storage) APIs to convert an image into text.

## win2d

[win2d](win2d) - An example using dependencies like [Win2D](https://www.nuget.org/packages/Win2D.uwp) where that component provides its own winmd for describing its API surface as well as a runtime DLL that must be deployed with the app. It is further complicated as Win2D requires the [VCRTForwarders](https://www.nuget.org/packages/Microsoft.VCRTForwarders.140/) in order to load. 

## xml

[xml](xml) - An example of using the [Windows.Data.Xml.Dom](https://docs.microsoft.com/en-us/uwp/api/Windows.Data.Xml.Dom) API for parsing XML.
