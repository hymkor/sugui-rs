// #![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn su(path: &str, param: &str) {
    let path_vec: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let param_vec: Vec<u16> = param.encode_utf16().chain(Some(0)).collect();

    unsafe {
        use windows::core::{w, PCWSTR};
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

        ShellExecuteW(
            HWND(0),
            w!("runas"),
            PCWSTR(path_vec.as_ptr()),
            PCWSTR(param_vec.as_ptr()),
            w!(""),
            SHOW_WINDOW_CMD(1),
        );
    }
}

fn mains() -> Option<String> {
    let mut args = std::env::args().skip(1);
    let path = match args.next() {
        None => return Some(String::from("few parameter")),
        Some(val) => val,
    };
    let param = match args.next() {
        None => String::from(""),
        Some(val) => val,
    };
    su(&path, &param);
    None
}

fn main() {
    if let Some(err) = mains() {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}
