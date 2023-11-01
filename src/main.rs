// #![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn make_pcwstr(s: &str) -> windows::core::PCWSTR {
    let v: Vec<u16> = s.encode_utf16().chain(Some(0).into_iter()).collect();
    windows::core::PCWSTR::from_raw(v.as_ptr())
}

fn su(path: &str, param: &str) {
    unsafe {
        use windows::core::w;
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        ShellExecuteW(
            HWND(0),
            w!("runas"),
            make_pcwstr(path),
            make_pcwstr(param),
            w!(""),
            SW_SHOWNORMAL,
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
