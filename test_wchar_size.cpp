#define IMGUI_USE_WCHAR32
#include "imgui-sys/third-party/imgui-docking-freetype/imgui/imgui.h"
#include <stdio.h>

int main() {
    printf("sizeof(ImWchar) = %zu\n", sizeof(ImWchar));
    printf("sizeof(ImFontConfig) = %zu\n", sizeof(ImFontConfig));
    printf("sizeof(ImGuiIO) = %zu\n", sizeof(ImGuiIO));
    printf("sizeof(ImFont) = %zu\n", sizeof(ImFont));
    printf("sizeof(ImGuiPlatformIO) = %zu\n", sizeof(ImGuiPlatformIO));
    printf("sizeof(ImGuiInputTextCallbackData) = %zu\n", sizeof(ImGuiInputTextCallbackData));
    return 0;
}
