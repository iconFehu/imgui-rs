use std::mem::{offset_of, size_of};

extern "C" {
    fn cpp_sizeof_ImWchar() -> usize;
    fn cpp_sizeof_ImFontConfig() -> usize;
    fn cpp_sizeof_ImGuiIO() -> usize;
    fn cpp_sizeof_ImFont() -> usize;
    fn cpp_sizeof_ImGuiPlatformIO() -> usize;
    fn cpp_sizeof_ImGuiInputTextCallbackData() -> usize;
    
    fn cpp_offset_ImFontConfig_FontData() -> usize;
    fn cpp_offset_ImFontConfig_OversampleH() -> usize;
    
    fn cpp_offset_ImGuiIO_DisplaySize() -> usize;
    fn cpp_offset_ImGuiIO_DeltaTime() -> usize;
    fn cpp_offset_ImGuiIO_MousePos() -> usize;
    fn cpp_offset_ImGuiIO_InputQueueCharacters() -> usize;
    
    fn cpp_offset_ImFont_EllipsisChar() -> usize;
    fn cpp_offset_ImFont_FallbackChar() -> usize;
    
    fn cpp_offset_ImGuiPlatformIO_Platform_LocaleDecimalPoint() -> usize;
    
    fn cpp_offset_ImGuiInputTextCallbackData_EventChar() -> usize;
}

#[test]
fn test_abi_compatibility() {
    unsafe {
        // Test sizeof - these verify WCHAR32 is correctly used
        assert_eq!(
            size_of::<imgui_sys::ImWchar>(),
            cpp_sizeof_ImWchar(),
            "ImWchar size mismatch (should be 4 bytes with WCHAR32)"
        );
        assert_eq!(
            size_of::<imgui_sys::ImFontConfig>(),
            cpp_sizeof_ImFontConfig(),
            "ImFontConfig size mismatch"
        );
        assert_eq!(
            size_of::<imgui_sys::ImGuiIO>(),
            cpp_sizeof_ImGuiIO(),
            "ImGuiIO size mismatch"
        );
        assert_eq!(
            size_of::<imgui_sys::ImFont>(),
            cpp_sizeof_ImFont(),
            "ImFont size mismatch"
        );
        assert_eq!(
            size_of::<imgui_sys::ImGuiPlatformIO>(),
            cpp_sizeof_ImGuiPlatformIO(),
            "ImGuiPlatformIO size mismatch"
        );
        assert_eq!(
            size_of::<imgui_sys::ImGuiInputTextCallbackData>(),
            cpp_sizeof_ImGuiInputTextCallbackData(),
            "ImGuiInputTextCallbackData size mismatch"
        );
        
        // Test key field offsets for ImFontConfig
        assert_eq!(
            offset_of!(imgui_sys::ImFontConfig, FontData),
            cpp_offset_ImFontConfig_FontData(),
            "ImFontConfig::FontData offset mismatch"
        );
        assert_eq!(
            offset_of!(imgui_sys::ImFontConfig, OversampleH),
            cpp_offset_ImFontConfig_OversampleH(),
            "ImFontConfig::OversampleH offset mismatch"
        );
        
        // Test key field offsets for ImGuiIO
        assert_eq!(
            offset_of!(imgui_sys::ImGuiIO, DisplaySize),
            cpp_offset_ImGuiIO_DisplaySize(),
            "ImGuiIO::DisplaySize offset mismatch"
        );
        assert_eq!(
            offset_of!(imgui_sys::ImGuiIO, DeltaTime),
            cpp_offset_ImGuiIO_DeltaTime(),
            "ImGuiIO::DeltaTime offset mismatch"
        );
        assert_eq!(
            offset_of!(imgui_sys::ImGuiIO, MousePos),
            cpp_offset_ImGuiIO_MousePos(),
            "ImGuiIO::MousePos offset mismatch"
        );
        assert_eq!(
            offset_of!(imgui_sys::ImGuiIO, InputQueueCharacters),
            cpp_offset_ImGuiIO_InputQueueCharacters(),
            "ImGuiIO::InputQueueCharacters offset mismatch"
        );
        
        // Test key field offsets for ImFont (verify ImWchar fields)
        assert_eq!(
            offset_of!(imgui_sys::ImFont, EllipsisChar),
            cpp_offset_ImFont_EllipsisChar(),
            "ImFont::EllipsisChar offset mismatch"
        );
        assert_eq!(
            offset_of!(imgui_sys::ImFont, FallbackChar),
            cpp_offset_ImFont_FallbackChar(),
            "ImFont::FallbackChar offset mismatch"
        );
        
        // Test key field offsets for ImGuiPlatformIO
        assert_eq!(
            offset_of!(imgui_sys::ImGuiPlatformIO, Platform_LocaleDecimalPoint),
            cpp_offset_ImGuiPlatformIO_Platform_LocaleDecimalPoint(),
            "ImGuiPlatformIO::Platform_LocaleDecimalPoint offset mismatch"
        );
        
        // Test key field offsets for ImGuiInputTextCallbackData
        assert_eq!(
            offset_of!(imgui_sys::ImGuiInputTextCallbackData, EventChar),
            cpp_offset_ImGuiInputTextCallbackData_EventChar(),
            "ImGuiInputTextCallbackData::EventChar offset mismatch"
        );
    }
}
