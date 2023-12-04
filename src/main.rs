use enigo::{Key, KeyboardControllable};

fn toggle_desktop(enigo: &mut enigo::Enigo) {
    enigo.key_down(Key::Meta);
    enigo.key_down(Key::D);
    enigo.key_up(Key::D);
    enigo.key_up(Key::Meta);
}

fn main() {
    let interval = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(20);
    let mut enigo = enigo::Enigo::new();
    println!("This program will press Win+D every {} minutes", interval);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(interval * 60));
        toggle_desktop(&mut enigo);
    }
}
