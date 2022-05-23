pub mod api;
mod error;

pub use api::Buffer;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[no_mangle]
pub extern "C" fn test() -> *mut std::os::raw::c_char {
    // api::create_buf(true, true).to_string().len().try_into().unwrap()

    // std::ffi::CString::new(api::get_current_buf().get_name())
    //     .unwrap()
    //     .into_raw()

    // api::replace_termcodes("<Cmd>q<CR>", true, true, true).unwrap().into_raw()

    // api::echo(
    //     [
    //         ("hey", Some("IncSearch")),
    //         (", this is some", None),
    //         ("Bullshiat", Some("DiffDelete")),
    //         ("Bullshiat", Some("ciaone")),
    //     ],
    //     true,
    // )
    // .unwrap()

    api::get_mode()
        .get::<_, nvim_types::NvimString>("mode")
        .unwrap()
        .as_c_str()
        .to_owned()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn is_modified() -> bool {
    api::get_current_buf().get_option::<bool>("modified").unwrap()
}

#[no_mangle]
pub extern "C" fn set_lines() {
    api::create_buf(true, false)
        .unwrap()
        .set_lines(0, 0, false, ["ciaone"])
        .unwrap()
}

#[no_mangle]
pub extern "C" fn set_option() -> bool {
    let mut buf = api::get_current_buf();
    buf.set_option("modified", true).unwrap();
    buf.get_option::<bool>("modified").unwrap()
}

#[no_mangle]
pub extern "C" fn set_var() -> bool {
    let mut buf = api::get_current_buf();
    buf.set_var("foo", true).unwrap();
    buf.get_var::<bool>("foo").unwrap()
}
