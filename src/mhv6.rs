use crate::extension;
use crate::state::State;
use std::ffi::CStr;
use std::{fs, ptr, thread};
use crate::utils::{download, next_free_id};
use clipboard_win::{Clipboard, get_clipboard_string};
use std::fs::OpenOptions;
use std::io::Write;

///
/// **MHV6.RS**
///
/// This file is meant for the GUI. I will provide examples so you can understand the insight.
/// Make sure to change any instances of "extension..." to your own mods name.
///
/// The top of the function renders last, while the bottom of the function renders first.
///
/// To find all the functions, refer to EXTENSION.RS.
/// I will update this repo whenever Absolute updates his MHV6 extensions repository.
///

pub fn mhv6_init() {
    while !extension::is_ready() {
        thread::sleep_ms(100);
    }
    let ext = extension::initialise_ext(b"NoNG'd\0".as_ptr());

    //This renders last.
    extension::add_button(ext, b"Download\0".as_ptr(), button_cb);

    extension::add_checkbox(ext, b"Auto ID\0".as_ptr(), checked_cb, unchecked_cb);

    let id_textbox = extension::add_textbox(ext, id_textbox_cb);
    extension::set_textbox_text(id_textbox, b"1085360\0".as_ptr());
    extension::set_textbox_placeholder(id_textbox, b"Song ID\0".as_ptr());

    let quality_combobox = extension::add_combobox(ext, quality_combobox_cb);
    extension::set_combobox_strs(
        quality_combobox,
        [
            b"128k\0".as_ptr(),
            b"192k\0".as_ptr(),
            b"256k\0".as_ptr(),
            b"320k\0".as_ptr(),
            ptr::null(),
        ]
            .as_mut_ptr(),
    );
    extension::set_combobox_index(quality_combobox, 0);

    let combobox = extension::add_combobox(ext, combobox_cb);
    extension::set_combobox_strs(
        combobox,
        [
            b"YouTube\0".as_ptr(),
            b"SoundCloud\0".as_ptr(),
            b"Spotify\0".as_ptr(),
            ptr::null()
        ]
            .as_mut_ptr(),
    );
    extension::set_combobox_index(combobox, 0);

    let textbox = extension::add_textbox(ext, textbox_cb);
    extension::set_textbox_text(textbox, b"ZmsdIQuywaE\0".as_ptr());
    extension::set_textbox_placeholder(textbox, b"Link\0".as_ptr());

    let version_textbox = extension::add_textbox(ext, version_textbox_cb);
    extension::set_textbox_text(version_textbox, b"\0".as_ptr());
    extension::set_textbox_placeholder(version_textbox, b"Version: 0.2.0\0\0".as_ptr());

    //This renders first.

    extension::commit_ext(ext);
}

extern "stdcall" fn button_cb(ext: *mut ()) {
    let mut state = State::get();
    if state.auto_id {
        download(&*state.link_ext, &*state.link_type, &*state.quality, next_free_id().as_str());
    } else {
        download(&*state.link_ext, &*state.link_type, &*state.quality, &*state.song_id);
    }
}

extern "stdcall" fn unchecked_cb(ext: *mut ()) {
    State::get().auto_id = false;
}

extern "stdcall" fn checked_cb(ext: *mut ()) {
    State::get().auto_id = true;
}

extern "stdcall" fn id_textbox_cb(ext: *mut ()) {
    let mut state = State::get();
    unsafe { state.song_id = CStr::from_ptr(extension::get_textbox_text(ext)).to_str().unwrap().to_string(); }
    if state.auto_id { extension::set_textbox_text(ext, next_free_id().as_ptr()); }
}

extern "stdcall" fn quality_combobox_cb(
    ext: *mut (),
    option_number: i32,
    name_of_option: *const u8,
) {
    unsafe {
        State::get().quality = CStr::from_ptr(name_of_option as *const i8).to_str().unwrap().to_string();
    }
}

extern "stdcall" fn combobox_cb(ext: *mut (), option_number: i32, name_of_option: *const u8) {
    let mut state = State::get();
    unsafe {
        state.link_type = CStr::from_ptr(name_of_option as *const i8).to_str().unwrap().to_string();
    }
}

extern "stdcall" fn textbox_cb(ext: *mut ()) {
    let mut state = State::get();
    unsafe { state.link_ext = CStr::from_ptr(extension::get_textbox_text(ext)).to_str().unwrap().to_string(); }
    if state.link_ext == ".pst" {
        state.link_ext = get_clipboard_string().unwrap();
        extension::set_textbox_text(ext, get_clipboard_string().unwrap().as_ptr());
    }
}

extern "stdcall" fn version_textbox_cb(ext: *mut ()) { extension::set_textbox_text(ext, "\0".as_ptr()); }