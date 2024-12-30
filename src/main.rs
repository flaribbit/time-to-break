use windows::{
    core::*,
    Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
    Win32::UI::Shell::{IShellDispatch4, Shell},
};

fn toggle_desktop(shell: &IShellDispatch4) -> Result<()> {
    unsafe { shell.ToggleDesktop() }
}

fn main() -> Result<()> {
    let interval = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(20);
    let shell: IShellDispatch4 = unsafe {
        CoInitialize(None).ok()?;
        CoCreateInstance(&Shell, None, CLSCTX_ALL)?
    };
    println!(
        "This program will toggle desktop every {} minutes",
        interval
    );
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval * 60));
        toggle_desktop(&shell)?;
    }
}
