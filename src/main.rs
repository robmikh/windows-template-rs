use bindings::Windows::UI::Colors;
use bindings::Windows::Win32::System::WinRT::{RoInitialize, RO_INIT_SINGLETHREADED};

fn main() -> windows::Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };

    let red = Colors::Red()?;
    println!("Red: {:?}", red);

    Ok(())
}
