fn su(path: &mut str, param: &mut str) {
    let mut path_vec: Vec<u16> = path.encode_utf16().collect();
    path_vec.push(0);

    let mut param_vec: Vec<u16> = param.encode_utf16().collect();
    param_vec.push(0);

    unsafe {
        use windows::core::{w, PCWSTR};
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

        ShellExecuteW(
            HWND(0),
            w!("runas"),
            PCWSTR(path_vec.as_mut_ptr()),
            PCWSTR(param_vec.as_mut_ptr()),
            w!(""),
            SHOW_WINDOW_CMD(0),
        );
    }
}

fn mains() -> Option<String> {
    let mut args = std::env::args().skip(1);
    let mut path = match args.next() {
        None => return Some(String::from("few parameter")),
        Some(val) => val,
    };
    let mut param = match args.next() {
        None => String::from(""),
        Some(val) => val,
    };
    su(&mut path, &mut param);
    None
}

fn main() {
    if let Some(err) = mains() {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}
