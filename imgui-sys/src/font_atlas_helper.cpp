// Helper functions for ImFontAtlas that cimgui doesn't expose
#include "imgui.h"

// ImFontAtlasBuildMain is the non-obsolete function that Build() calls
// It's always available, even when IMGUI_DISABLE_OBSOLETE_FUNCTIONS is set
extern void ImFontAtlasBuildMain(ImFontAtlas* atlas);

extern "C" {
    void ImFontAtlas_BuildMain_Wrapper(ImFontAtlas* atlas) {
        ImFontAtlasBuildMain(atlas);
    }
}
