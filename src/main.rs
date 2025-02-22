use windows::{
    core::Result,
    Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER},
    Win32::UI::Shell::{IShellDispatch4, Shell},
};

fn main() -> Result<()> {
    let interval = std::env::args()
        .nth(1)
        .map(|s| s.parse::<f32>().unwrap())
        .unwrap_or(20.0);
    let shell: IShellDispatch4 = unsafe {
        CoInitialize(None).ok()?;
        CoCreateInstance(&Shell, None, CLSCTX_INPROC_SERVER)?
    };
    let duration = std::time::Duration::from_secs((interval * 60.0) as u64);
    println!(
        "This program will minimize all windows every {} minutes.",
        interval
    );
    loop {
        std::thread::sleep(duration);
        notify_rust::Notification::new()
            .summary("time-to-break")
            .body("You should take a break!")
            .show()
            .ok();
        unsafe { shell.MinimizeAll() }.ok();
    }
}
