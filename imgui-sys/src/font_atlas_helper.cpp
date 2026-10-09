// Helper functions for ImFontAtlas that cimgui doesn't expose
#include "imgui.h"

extern "C" {
    bool ImFontAtlas_Build_Wrapper(ImFontAtlas* atlas) {
        return atlas->Build();
    }
}
