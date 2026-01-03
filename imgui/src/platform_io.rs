use std::ffi::{c_char, c_void};

use crate::internal::RawCast;

/// Holds the information needed to enable multiple viewports.
#[repr(C)]
pub struct PlatformIo {
    pub(crate) get_clipboard_text_fn:
        Option<unsafe extern "C" fn(*mut sys::ImGuiContext) -> *const c_char>,

    pub(crate) set_clipboard_text_fn:
        Option<unsafe extern "C" fn(*mut sys::ImGuiContext, *const c_char)>,

    pub(crate) clipboard_user_data: *mut c_void,

    pub(crate) open_in_shell_fn:
        Option<unsafe extern "C" fn(ctx: *mut sys::ImGuiContext, path: *const c_char) -> bool>,
    pub(crate) open_in_shell_user_data: *mut c_void,
    pub(crate) set_ime_data_fn: Option<
        unsafe extern "C" fn(
            ctx: *mut sys::ImGuiContext,
            viewport: *mut sys::ImGuiViewport,
            data: *mut sys::ImGuiPlatformImeData,
        ),
    >,
    pub(crate) ime_user_data: *mut c_void,
    pub(crate) locale_decimal_point: sys::ImWchar,
    pub renderer_texture_max_width: i32,
    pub renderer_texture_max_height: i32,
    pub renderer_render_state: *mut c_void,
    pub textures: sys::ImVector_ImTextureDataPtr,
}

unsafe impl RawCast<sys::ImGuiPlatformIO> for PlatformIo {}

#[test]
#[cfg(test)]
fn test_platform_io_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<PlatformIo>(),
        mem::size_of::<sys::ImGuiPlatformIO>()
    );
    assert_eq!(
        mem::align_of::<PlatformIo>(),
        mem::align_of::<sys::ImGuiPlatformIO>()
    );
    use sys::ImGuiPlatformIO;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(PlatformIo, $l),
                memoffset::offset_of!(ImGuiPlatformIO, $r)
            );
        };
    }

    assert_field_offset!(get_clipboard_text_fn, Platform_GetClipboardTextFn);
    assert_field_offset!(set_clipboard_text_fn, Platform_SetClipboardTextFn);
    assert_field_offset!(clipboard_user_data, Platform_ClipboardUserData);
    assert_field_offset!(open_in_shell_fn, Platform_OpenInShellFn);
    assert_field_offset!(open_in_shell_user_data, Platform_OpenInShellUserData);
    assert_field_offset!(set_ime_data_fn, Platform_SetImeDataFn);
    assert_field_offset!(ime_user_data, Platform_ImeUserData);
    assert_field_offset!(locale_decimal_point, Platform_LocaleDecimalPoint);
    assert_field_offset!(renderer_texture_max_width, Renderer_TextureMaxWidth);
    assert_field_offset!(renderer_texture_max_height, Renderer_TextureMaxHeight);
    assert_field_offset!(renderer_render_state, Renderer_RenderState);
    assert_field_offset!(textures, Textures);
}

/// Describes an ImGui Viewport.
#[repr(C)]
pub struct Viewport {
    /// The unique ID of this Viewport.
    pub id: crate::Id,
    /// Flags that describe how the Viewport should behave.
    pub flags: crate::ViewportFlags,
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub framebuffer_scale: [f32; 2],
    pub work_pos: [f32; 2],
    pub work_size: [f32; 2],
    pub platform_handle: *mut c_void,
    pub platform_handle_raw: *mut c_void,
}

#[test]
#[cfg(test)]
fn test_viewport_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<Viewport>(),
        mem::size_of::<sys::ImGuiViewport>()
    );
    assert_eq!(
        mem::align_of::<Viewport>(),
        mem::align_of::<sys::ImGuiViewport>()
    );
    use sys::ImGuiViewport;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(Viewport, $l),
                memoffset::offset_of!(ImGuiViewport, $r)
            );
        };
    }

    assert_field_offset!(id, ID);
    assert_field_offset!(flags, Flags);
    assert_field_offset!(pos, Pos);
    assert_field_offset!(size, Size);
    assert_field_offset!(framebuffer_scale, FramebufferScale);
    assert_field_offset!(work_pos, WorkPos);
    assert_field_offset!(work_size, WorkSize);
    assert_field_offset!(platform_handle, PlatformHandle);
    assert_field_offset!(platform_handle_raw, PlatformHandleRaw);
}
