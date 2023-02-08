use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;
use windows::Gaming::Input::RawGameController;
use windows::Win32::Graphics::Gdi::{RedrawWindow, RDW_INVALIDATE};
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

// Most of this is taken from the create_window example in the Windows crate:
// https://github.com/microsoft/windows-rs/blob/4726348167316b4624abbe57e0b09cd05f12e0d5/crates/samples/create_window/src/main.rs
// See other code comments for differences.

// Store gamepads we have already found so we can print when a new one is connected.
lazy_static! {
    static ref FOUND_GAMEPADS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                // Call our code that prints the controller names. The exception happens in here.
                print_gamepads();
                ValidateRect(window, None);
                // The exception doesn't happen on the first frame because WGI always says there
                // are no controllers the first time you ask. Request new frames so it fails
                // without having to resize the window to get it to run again.
                RedrawWindow(window, None, None, RDW_INVALIDATE);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

fn print_gamepads() {
    let gamepads = match RawGameController::RawGameControllers() {
        Ok(gamepads) => gamepads,
        Err(e) => panic!("Error while fetching gamepads {e}"),
    };

    match gamepads.Size() {
        Ok(0)
            if !FOUND_GAMEPADS
                .lock()
                .map(|fg| fg.contains("None"))
                .unwrap_or_default() =>
        {
            println!("No Gamepads found at startup");
            if let Ok(mut found_gamepads) = FOUND_GAMEPADS.lock() {
                found_gamepads.insert("None".to_string());
            }
        }
        _ => {
            for controller in gamepads {
                let raw_game_controllers = RawGameController::RawGameControllers().unwrap();
                let mut index = 0;
                raw_game_controllers
                    .IndexOf(&controller, &mut index)
                    .unwrap();
                let name = match controller.DisplayName() {
                    Ok(hstring) => hstring.to_string_lossy(),
                    Err(_) => "unknown".to_string(),
                };
                match controller.NonRoamableId() {
                    Ok(id) => {
                        let id = id.to_string_lossy();
                        if let Ok(mut found_gamepads) = FOUND_GAMEPADS.lock() {
                            if !found_gamepads.contains(id.as_str()) {
                                println!("Found controller: {name}, {id}, index: {index}");
                                found_gamepads.insert(id);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Found controller: {name}, Error getting id: {e}, index: {index}");
                    }
                }
            }
        }
    }
}
