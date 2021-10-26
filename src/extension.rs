use getfn::symbol_fn;

///
/// **EXTENSION.RS**
///
/// This file contains all the necessary functions to create a working MHV6 GUI.
/// It's implemented in the file MHV6.RS, you can see my examples there.
///

symbol_fn! {
    ("hackpro.dll" @ "?HackproIsReady@@YA_NXZ")
    pub extern "C" fn is_ready() -> bool;
    ("hackpro.dll" @ "?HackproInitialiseExt@@YAPAXPBD@Z")
    pub extern "C" fn initialise_ext(name: *const u8) -> *mut ();
    ("hackpro.dll" @ "?HackproAddButton@@YA_NPAXPBDP6GX0@Z@Z")
    pub extern "C" fn add_button(ptr: *mut (), str: *const u8, callback: extern "stdcall" fn(*mut ())) -> bool;
    ("hackpro.dll" @ "?HackproAddCheckbox@@YAPAXPAXPBDP6GX0@Z2@Z")
    pub extern "C" fn add_checkbox(ptr: *mut (), str: *const u8, checked_callback: extern "stdcall" fn(*mut ()), unchecked_callback: extern "stdcall" fn(*mut ())) -> *mut ();
    ("hackpro.dll" @ "?HackproSetCheckbox@@YA_NPAX_N@Z")
    pub extern "C" fn set_checkbox(checkbox: *mut (), state: bool) -> bool;
    ("hackpro.dll" @ "?HackproAddComboBox@@YAPAXPAXP6GX0HPBD@Z@Z")
    pub extern "C" fn add_combobox(ptr: *mut (), callback: extern "stdcall" fn(*mut (), i32, *const u8)) -> *mut ();
    ("hackpro.dll" @ "?HackproSetComboBoxStrs@@YA_NPAXPAPBD@Z")
    pub extern "C" fn set_combobox_strs(combo: *mut (), strs: *const *const u8) -> bool;
    ("hackpro.dll" @ "?HackproSetComboBoxIndex@@YA_NPAXH@Z")
    pub extern "C" fn set_combobox_index(combo: *mut (), idx: i32) -> bool;
    ("hackpro.dll" @ "?HackproAddTextBox@@YAPAXPAXP6GX0@Z@Z")
    pub extern "C" fn add_textbox(ptr: *mut (), callback: extern "stdcall" fn(*mut ())) -> *mut ();
    ("hackpro.dll" @ "?HackproSetTextBoxText@@YA_NPAXPBD@Z")
    pub extern "C" fn set_textbox_text(b: *mut (), str: *const u8) -> bool;
    ("hackpro.dll" @ "?HackproSetTextBoxPlaceholder@@YA_NPAXPBD@Z")
    pub extern "C" fn set_textbox_placeholder(b: *mut (), str: *const u8);
    ("hackpro.dll" @ "?HackproGetTextBoxText@@YAPBDPAX@Z")
    pub extern "C" fn get_textbox_text(b: *mut ()) -> *const i8;

    ("hackpro.dll" @ "?HackproSetUserData@@YAXPAX0@Z")
    pub extern "C" fn set_user_data(ptr: *mut (), data: *mut ());
    ("hackpro.dll" @ "?HackproGetUserData@@YAPAXPAX@Z")
    pub extern "C" fn get_user_data(ptr: *mut ()) -> *mut ();

    ("hackpro.dll" @ "?HackproCommitExt@@YA_NPAX@Z")
    pub extern "C" fn commit_ext(ptr: *mut ()) -> bool;
    ("hackpro.dll" @ "?HackproWithdrawExt@@YA_NPAX@Z")
    pub extern "C" fn withdraw_ext(ptr: *mut ()) -> bool;

}
