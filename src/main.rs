use bindings::Windows::UI::Colors;

fn main() -> windows::Result<()> {
    windows::initialize_sta()?;

    let red = Colors::Red()?;
    println!("Red: {:?}", red);

    Ok(())
}
