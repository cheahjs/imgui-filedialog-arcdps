//! Raw FFI bindings to ImGuiFileDialog v0.5.4
//!
//! This crate provides low-level bindings to the ImGuiFileDialog C API.
//! For a safe, idiomatic Rust API, use the `imgui-filedialog` crate instead.
//!
//! Note: This version targets ImGuiFileDialog v0.5.4 which is compatible
//! with ImGui 1.80 (as used by arcdps-imgui).

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]

use std::os::raw::{c_char, c_float, c_int, c_void};

// Re-export imgui types we need
pub use arcdps_imgui_sys::{ImGuiWindowFlags, ImVec2, ImVec4};

/// Opaque file dialog context
#[repr(C)]
pub struct ImGuiFileDialog {
    _private: [u8; 0],
}

/// File dialog flags
pub type ImGuiFileDialogFlags = c_int;
pub const ImGuiFileDialogFlags_None: ImGuiFileDialogFlags = 0;
pub const ImGuiFileDialogFlags_ConfirmOverwrite: ImGuiFileDialogFlags = 1 << 0;

/// Callback function type for custom side pane
pub type IGFD_PaneFun = Option<unsafe extern "C" fn(*const c_char, *mut c_void, *mut bool)>;

/// A single selection item (filename + full path)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IGFD_Selection_Pair {
    pub fileName: *mut c_char,
    pub filePathName: *mut c_char,
}

/// Collection of selected files
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IGFD_Selection {
    pub table: *mut IGFD_Selection_Pair,
    pub count: usize,
}

extern "C" {
    // ============================================================
    // Selection helpers
    // ============================================================

    /// Get an initialized selection pair
    pub fn IGFD_Selection_Pair_Get() -> IGFD_Selection_Pair;

    /// Destroy the content of a selection pair (frees strings)
    pub fn IGFD_Selection_Pair_DestroyContent(pair: *mut IGFD_Selection_Pair);

    /// Get an initialized selection
    pub fn IGFD_Selection_Get() -> IGFD_Selection;

    /// Destroy the content of a selection (frees all pairs)
    pub fn IGFD_Selection_DestroyContent(selection: *mut IGFD_Selection);

    // ============================================================
    // Construction / Destruction
    // ============================================================

    /// Create a new file dialog context
    pub fn IGFD_Create() -> *mut ImGuiFileDialog;

    /// Destroy a file dialog context
    pub fn IGFD_Destroy(ctx: *mut ImGuiFileDialog);

    // ============================================================
    // Open Dialog Functions
    // ============================================================

    /// Open a standard dialog
    pub fn IGFD_OpenDialog(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        path: *const c_char,
        file_name: *const c_char,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a standard dialog with path extracted from file_path_name
    pub fn IGFD_OpenDialog2(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        file_path_name: *const c_char,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a standard dialog with custom side pane
    pub fn IGFD_OpenPaneDialog(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        path: *const c_char,
        file_name: *const c_char,
        side_pane: IGFD_PaneFun,
        side_pane_width: c_float,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a standard dialog with custom side pane - path extracted from file_path_name
    pub fn IGFD_OpenPaneDialog2(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        file_path_name: *const c_char,
        side_pane: IGFD_PaneFun,
        side_pane_width: c_float,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a modal dialog
    pub fn IGFD_OpenModal(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        path: *const c_char,
        file_name: *const c_char,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a modal dialog - path extracted from file_path_name
    pub fn IGFD_OpenModal2(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        file_path_name: *const c_char,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a modal dialog with custom side pane
    pub fn IGFD_OpenPaneModal(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        path: *const c_char,
        file_name: *const c_char,
        side_pane: IGFD_PaneFun,
        side_pane_width: c_float,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    /// Open a modal dialog with custom side pane - path extracted from file_path_name
    pub fn IGFD_OpenPaneModal2(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        title: *const c_char,
        filters: *const c_char,
        file_path_name: *const c_char,
        side_pane: IGFD_PaneFun,
        side_pane_width: c_float,
        count_selection_max: c_int,
        user_datas: *mut c_void,
        flags: ImGuiFileDialogFlags,
    );

    // ============================================================
    // Dialog Display / Control
    // ============================================================

    /// Display the dialog
    /// Returns true while the dialog is visible
    pub fn IGFD_DisplayDialog(
        ctx: *mut ImGuiFileDialog,
        key: *const c_char,
        flags: ImGuiWindowFlags,
        min_size: ImVec2,
        max_size: ImVec2,
    ) -> bool;

    /// Close the dialog
    pub fn IGFD_CloseDialog(ctx: *mut ImGuiFileDialog);

    /// Returns true if dialog was closed with OK
    pub fn IGFD_IsOk(ctx: *mut ImGuiFileDialog) -> bool;

    /// Check if the dialog key was opened this frame
    pub fn IGFD_WasKeyOpenedThisFrame(ctx: *mut ImGuiFileDialog, key: *const c_char) -> bool;

    /// Check if any dialog was opened this frame
    pub fn IGFD_WasOpenedThisFrame(ctx: *mut ImGuiFileDialog) -> bool;

    /// Check if a specific dialog key is currently opened
    pub fn IGFD_IsKeyOpened(ctx: *mut ImGuiFileDialog, key: *const c_char) -> bool;

    /// Check if any dialog is currently opened
    pub fn IGFD_IsOpened(ctx: *mut ImGuiFileDialog) -> bool;

    // ============================================================
    // Results
    // ============================================================

    /// Get the selection (multi-select mode)
    /// The returned selection must be freed with IGFD_Selection_DestroyContent
    pub fn IGFD_GetSelection(ctx: *mut ImGuiFileDialog) -> IGFD_Selection;

    /// Get the full file path (save mode)
    /// WARNING: You are responsible for freeing the returned string
    pub fn IGFD_GetFilePathName(ctx: *mut ImGuiFileDialog) -> *mut c_char;

    /// Get just the filename (save mode)
    /// WARNING: You are responsible for freeing the returned string
    pub fn IGFD_GetCurrentFileName(ctx: *mut ImGuiFileDialog) -> *mut c_char;

    /// Get the current path
    /// WARNING: You are responsible for freeing the returned string
    pub fn IGFD_GetCurrentPath(ctx: *mut ImGuiFileDialog) -> *mut c_char;

    /// Get the current filter
    /// WARNING: You are responsible for freeing the returned string
    pub fn IGFD_GetCurrentFilter(ctx: *mut ImGuiFileDialog) -> *mut c_char;

    /// Get user data passed when opening the dialog
    pub fn IGFD_GetUserDatas(ctx: *mut ImGuiFileDialog) -> *mut c_void;

    // ============================================================
    // Extension Info (file styling)
    // ============================================================

    /// Set extension display info with color and optional icon
    pub fn IGFD_SetExtentionInfos(
        ctx: *mut ImGuiFileDialog,
        filter: *const c_char,
        color: ImVec4,
        icon_text: *const c_char,
    );

    /// Set extension display info with explicit RGBA values
    pub fn IGFD_SetExtentionInfos2(
        ctx: *mut ImGuiFileDialog,
        filter: *const c_char,
        r: c_float,
        g: c_float,
        b: c_float,
        a: c_float,
        icon_text: *const c_char,
    );

    /// Get extension display info
    pub fn IGFD_GetExtentionInfos(
        ctx: *mut ImGuiFileDialog,
        filter: *const c_char,
        out_color: *mut ImVec4,
        out_icon_text: *mut *mut c_char,
    ) -> bool;

    /// Clear all extension settings
    pub fn IGFD_ClearExtentionInfos(ctx: *mut ImGuiFileDialog);
}

// ============================================================
// Exploration by Keys (optional feature)
// ============================================================
#[cfg(feature = "exploration_by_keys")]
extern "C" {
    /// Set the flashing attenuation time for keyboard navigation
    pub fn IGFD_SetFlashingAttenuationInSeconds(ctx: *mut ImGuiFileDialog, seconds: c_float);
}

// ============================================================
// Bookmarks (optional feature) - USE_BOOKMARK in v0.5.4
// ============================================================
#[cfg(feature = "bookmark")]
extern "C" {
    /// Serialize bookmarks to a string for saving
    /// WARNING: You are responsible for freeing the returned string
    pub fn IGFD_SerializeBookmarks(ctx: *mut ImGuiFileDialog) -> *mut c_char;

    /// Deserialize bookmarks from a saved string
    pub fn IGFD_DeserializeBookmarks(ctx: *mut ImGuiFileDialog, bookmarks: *const c_char);
}
