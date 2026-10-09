#define IMGUI_USE_WCHAR32
#include "imgui.h"
#include <stdio.h>
#include <stddef.h>

extern "C" {
    size_t cpp_sizeof_ImWchar() { return sizeof(ImWchar); }
    size_t cpp_sizeof_ImFontConfig() { return sizeof(ImFontConfig); }
    size_t cpp_sizeof_ImGuiIO() { return sizeof(ImGuiIO); }
    size_t cpp_sizeof_ImFont() { return sizeof(ImFont); }
    size_t cpp_sizeof_ImGuiPlatformIO() { return sizeof(ImGuiPlatformIO); }
    size_t cpp_sizeof_ImGuiInputTextCallbackData() { return sizeof(ImGuiInputTextCallbackData); }
    
    size_t cpp_offset_ImFontConfig_FontData() { return offsetof(ImFontConfig, FontData); }
    size_t cpp_offset_ImFontConfig_OversampleH() { return offsetof(ImFontConfig, OversampleH); }
    
    size_t cpp_offset_ImGuiIO_DisplaySize() { return offsetof(ImGuiIO, DisplaySize); }
    size_t cpp_offset_ImGuiIO_DeltaTime() { return offsetof(ImGuiIO, DeltaTime); }
    size_t cpp_offset_ImGuiIO_MousePos() { return offsetof(ImGuiIO, MousePos); }
    size_t cpp_offset_ImGuiIO_InputQueueCharacters() { return offsetof(ImGuiIO, InputQueueCharacters); }
    
    size_t cpp_offset_ImFont_EllipsisChar() { return offsetof(ImFont, EllipsisChar); }
    size_t cpp_offset_ImFont_FallbackChar() { return offsetof(ImFont, FallbackChar); }
    
    size_t cpp_offset_ImGuiPlatformIO_Platform_LocaleDecimalPoint() { 
        return offsetof(ImGuiPlatformIO, Platform_LocaleDecimalPoint); 
    }
    
    size_t cpp_offset_ImGuiInputTextCallbackData_EventChar() { 
        return offsetof(ImGuiInputTextCallbackData, EventChar); 
    }
}
