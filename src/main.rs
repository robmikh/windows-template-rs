use bindings::windows::ui::Colors;

fn main() -> windows::Result<()> {
    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
