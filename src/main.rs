use bindings::windows::ui::Colors;

fn main() -> windows::Result<()> {
    windows::initialize_sta()?;

    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
