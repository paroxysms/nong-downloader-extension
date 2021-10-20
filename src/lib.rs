#![feature(abi_thiscall, core_intrinsics, const_float_bits_conv)]

mod mhv6;
mod extension;

use std::{panic, thread};

pub use winwrap::winapi as __getfn_winapi__;

use winwrap::winapi::shared::minwindef::*;
use winwrap::winapi::shared::minwindef::{FALSE, TRUE}; // override ntdef
use winwrap::winapi::shared::ntdef::*;
use winwrap::winapi::um::wincon::*;
use winwrap::winapi::um::winnt::*;
use winwrap::raw::um::consoleapi::*;
use winwrap::raw::um::libloaderapi::*;
use winwrap::raw::um::processthreadsapi::*;
use winwrap::winapi::um::libloaderapi::GetModuleHandleA;
use crate::mhv6::mhv6_init;

///
/// **MEGAHACKv6 EXTENSION**
///
/// **If you want to rename your function, change anything relating to "extension..."**
///
/// "DllMain" is a necessary function for attaching the DLL to Geometry Dash.
/// If you decide to rename your extension (which I recommend), make sure to refactor it safely.
///
/// -----
///
/// "extension_main" is your extensions main controller function. You can check for quite a few errors with it.
///
/// If you would like to enable a console for debugging, uncomment the following lines:
/// Line #27: let _ = AllocConsole();
/// Line #40: FreeLibraryAndExitThread(dll as _, 0);
///
/// If you would like to add to the error function, just put it between line #34 and line #38
///

#[no_mangle]
pub extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            let result = CreateThread(0 as _, 0, Some(extension_main), dll as _, 0, 0 as _);
            if result.is_err() {
                return FALSE;
            }
        }
    }

    TRUE
}

unsafe extern "system" fn extension_main(dll: LPVOID) -> DWORD {
    if panic::catch_unwind(|| {
        //let _ = AllocConsole(); - Uncomment for enabling console.

        //State::init();

        extension_main_internal();
    })
        .is_err()
    {
        /*
        Enter an error message here. If the extension crashes, the following will be run.
        */
    }

    //FreeLibraryAndExitThread(dll as _, 0); - Uncomment for enabling console.

    0
}

fn extension_main_internal() {
    unsafe {
        if !GetModuleHandleA(b"hackpro.dll\0".as_ptr() as *const i8).is_null() {
            thread::spawn(|| mhv6_init());
        }
    }
}