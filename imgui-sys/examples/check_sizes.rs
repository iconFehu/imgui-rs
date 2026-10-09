use std::mem::size_of;
fn main() {
    println!("Rust sizeof(ImWchar) = {}", size_of::<imgui_sys::ImWchar>());
    println!("Rust sizeof(ImFontConfig) = {}", size_of::<imgui_sys::ImFontConfig>());
    println!("Rust sizeof(ImGuiIO) = {}", size_of::<imgui_sys::ImGuiIO>());
    println!("Rust sizeof(ImFont) = {}", size_of::<imgui_sys::ImFont>());
    println!("Rust sizeof(ImGuiPlatformIO) = {}", size_of::<imgui_sys::ImGuiPlatformIO>());
    println!("Rust sizeof(ImGuiInputTextCallbackData) = {}", size_of::<imgui_sys::ImGuiInputTextCallbackData>());
}
