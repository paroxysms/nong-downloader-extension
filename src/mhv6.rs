use std::{thread, fs, ptr};
use crate::extension;

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
    while !extension::is_ready() { thread::sleep_ms(100); }
    let ext = extension::initialise_ext(b"Extension\0".as_ptr());

    //This renders last.

    let textbox = extension::add_textbox(ext, textbox_cb);
    extension::set_textbox_text(textbox, b"Text\0".as_ptr());
    extension::set_textbox_placeholder(textbox, b"Placeholder Text\0".as_ptr());

    let combobox = extension::add_combobox(ext, combobox_cb);
    extension::set_combobox_strs(combobox, [b"Option 1\0".as_ptr(), b"Option 2\0".as_ptr(), ptr::null()].as_mut_ptr());
    extension::set_combobox_index(combobox, 0);

    let checkbox = extension::add_checkbox(
        ext,
        b"Checkbox\0".as_ptr(),
        checkbox_checked_cb,
        checkbox_unchecked_cb,
    );
    extension::set_checkbox(checkbox, true);

    extension::add_button(ext, b"Button\0".as_ptr(), button_cb);

    //This renders first.

    extension::commit_ext(ext);
}

extern "stdcall" fn button_cb(ext: *mut ()) {
    println!("Button pressed!");
}

extern "stdcall" fn checkbox_checked_cb(ext: *mut ()) {
    println!("Checkbox checked!");
}

extern "stdcall" fn checkbox_unchecked_cb(ext: *mut ()) {
    println!("Checkbox unchecked!");
}

extern "stdcall" fn combobox_cb(ext: *mut (), option_number: i32, name_of_option: *const u8) {
    println!("Combobox {:?}:{:?}!", name_of_option, option_number);
}

extern "stdcall" fn textbox_cb(ext: *mut ()) {
    println!("Textbox {:?}!", extension::get_textbox_text(ext));
}