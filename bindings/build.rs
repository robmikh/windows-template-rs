fn main() {
    windows::build!(
        Windows::UI::{
            Color, Colors,
        },
        Windows::Win32::System::WinRT::{
            RoInitialize,
        },
    );
}
