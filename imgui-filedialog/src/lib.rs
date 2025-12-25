//! Safe Rust bindings to ImGuiFileDialog v0.5.4 for arcdps-imgui
//!
//! This crate provides idiomatic Rust bindings for the [ImGuiFileDialog](https://github.com/aiekick/ImGuiFileDialog)
//! library, designed for use with [arcdps-imgui](https://crates.io/crates/arcdps-imgui).
//!
//! Note: This version targets ImGuiFileDialog v0.5.4 which is compatible with ImGui 1.80.
//!
//! # Example
//!
//! ```no_run
//! use imgui_filedialog::FileDialog;
//!
//! // Create a dialog instance (typically stored in your app state)
//! let mut dialog = FileDialog::new();
//!
//! // Open the dialog (e.g., when a button is clicked)
//! dialog.open_file()
//!     .title("Select a File")
//!     .filters(".txt,.md,.rs")
//!     .path(".")
//!     .build("choose_file");
//!
//! // In your render loop:
//! if dialog.display("choose_file", [400.0, 300.0], [800.0, 600.0]) {
//!     if dialog.is_ok() {
//!         if let Some(selection) = dialog.selection() {
//!             for path in selection.files() {
//!                 println!("Selected: {:?}", path);
//!             }
//!         }
//!     }
//!     dialog.close();
//! }
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

pub use imgui_filedialog_sys as sys;

/// A file dialog context.
///
/// This wraps the ImGuiFileDialog C++ class and manages its lifetime.
/// Create one instance and reuse it for the lifetime of your application.
pub struct FileDialog {
    ptr: *mut sys::ImGuiFileDialog,
}

impl Default for FileDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl FileDialog {
    /// Create a new file dialog context.
    pub fn new() -> Self {
        let ptr = unsafe { sys::IGFD_Create() };
        Self { ptr }
    }

    /// Open a file selection dialog.
    ///
    /// Returns a builder to configure the dialog before opening.
    pub fn open_file(&mut self) -> FileDialogBuilder<'_> {
        FileDialogBuilder::new(self, DialogMode::OpenFile)
    }

    /// Open a directory selection dialog.
    ///
    /// Returns a builder to configure the dialog before opening.
    pub fn open_directory(&mut self) -> FileDialogBuilder<'_> {
        FileDialogBuilder::new(self, DialogMode::OpenDirectory)
    }

    /// Open a save file dialog.
    ///
    /// Returns a builder to configure the dialog before opening.
    pub fn save_file(&mut self) -> FileDialogBuilder<'_> {
        FileDialogBuilder::new(self, DialogMode::SaveFile)
    }

    /// Display the dialog.
    ///
    /// Call this every frame in your render loop.
    /// Returns `true` while the dialog is visible (not yet closed).
    ///
    /// # Arguments
    /// * `key` - The same key used when opening the dialog
    /// * `min_size` - Minimum dialog size `[width, height]`
    /// * `max_size` - Maximum dialog size `[width, height]`
    pub fn display(&mut self, key: &str, min_size: [f32; 2], max_size: [f32; 2]) -> bool {
        let key_c = CString::new(key).unwrap();
        unsafe {
            sys::IGFD_DisplayDialog(
                self.ptr,
                key_c.as_ptr(),
                0,
                sys::ImVec2 {
                    x: min_size[0],
                    y: min_size[1],
                },
                sys::ImVec2 {
                    x: max_size[0],
                    y: max_size[1],
                },
            )
        }
    }

    /// Returns `true` if the user clicked OK (confirmed selection).
    pub fn is_ok(&self) -> bool {
        unsafe { sys::IGFD_IsOk(self.ptr) }
    }

    /// Check if any dialog is currently open.
    pub fn is_opened(&self) -> bool {
        unsafe { sys::IGFD_IsOpened(self.ptr) }
    }

    /// Check if a specific dialog key is open.
    pub fn is_key_opened(&self, key: &str) -> bool {
        let key_c = CString::new(key).unwrap();
        unsafe { sys::IGFD_IsKeyOpened(self.ptr, key_c.as_ptr()) }
    }

    /// Close the dialog.
    pub fn close(&mut self) {
        unsafe { sys::IGFD_CloseDialog(self.ptr) }
    }

    /// Get the selected files.
    ///
    /// Returns `None` if the dialog was cancelled or no files were selected.
    pub fn selection(&self) -> Option<Selection> {
        if !self.is_ok() {
            return None;
        }

        let selection = unsafe { sys::IGFD_GetSelection(self.ptr) };

        Some(Selection::new(selection))
    }

    /// Get the full file path (for save dialogs).
    ///
    /// Returns `None` if the dialog was cancelled.
    pub fn file_path_name(&self) -> Option<PathBuf> {
        if !self.is_ok() {
            return None;
        }

        unsafe {
            let ptr = sys::IGFD_GetFilePathName(self.ptr);
            if ptr.is_null() {
                return None;
            }
            let path = ptr_to_pathbuf(ptr);
            libc::free(ptr as *mut _);
            Some(path)
        }
    }

    /// Get the current directory path.
    pub fn current_path(&self) -> Option<PathBuf> {
        unsafe {
            let ptr = sys::IGFD_GetCurrentPath(self.ptr);
            if ptr.is_null() {
                return None;
            }
            let path = ptr_to_pathbuf(ptr);
            libc::free(ptr as *mut _);
            Some(path)
        }
    }

    /// Get the current filter.
    pub fn current_filter(&self) -> Option<String> {
        unsafe {
            let ptr = sys::IGFD_GetCurrentFilter(self.ptr);
            if ptr.is_null() {
                return None;
            }
            let s = ptr_to_string(ptr);
            libc::free(ptr as *mut _);
            Some(s)
        }
    }

    /// Set a custom file extension style.
    ///
    /// # Arguments
    /// * `extension` - Extension filter (e.g., ".txt", ".rs")
    /// * `color` - RGBA color `[r, g, b, a]`
    /// * `icon` - Optional icon/text prefix
    pub fn set_extension_infos(&mut self, extension: &str, color: [f32; 4], icon: Option<&str>) {
        let ext_c = CString::new(extension).unwrap();
        let icon_c = icon.map(|s| CString::new(s).unwrap());
        let icon_ptr = icon_c
            .as_ref()
            .map(|c| c.as_ptr())
            .unwrap_or(std::ptr::null());

        unsafe {
            sys::IGFD_SetExtentionInfos(
                self.ptr,
                ext_c.as_ptr(),
                sys::ImVec4 {
                    x: color[0],
                    y: color[1],
                    z: color[2],
                    w: color[3],
                },
                icon_ptr,
            );
        }
    }

    /// Clear all extension settings.
    pub fn clear_extension_infos(&mut self) {
        unsafe { sys::IGFD_ClearExtentionInfos(self.ptr) }
    }

    /// Get the raw FFI pointer.
    ///
    /// # Safety
    /// The caller must ensure the FileDialog outlives any use of this pointer.
    pub fn as_ptr(&self) -> *mut sys::ImGuiFileDialog {
        self.ptr
    }
}

impl Drop for FileDialog {
    fn drop(&mut self) {
        unsafe { sys::IGFD_Destroy(self.ptr) }
    }
}

// SAFETY: ImGuiFileDialog is not thread-safe, but it's safe to send between threads
// as long as it's not accessed concurrently.
unsafe impl Send for FileDialog {}

/// Builder for configuring a file dialog before opening.
pub struct FileDialogBuilder<'a> {
    dialog: &'a mut FileDialog,
    mode: DialogMode,
    title: Option<CString>,
    filters: Option<CString>,
    path: Option<CString>,
    file_name: Option<CString>,
    max_selection: i32,
    modal: bool,
    flags: sys::ImGuiFileDialogFlags,
}

/// Dialog mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogMode {
    OpenFile,
    OpenDirectory,
    SaveFile,
}

impl<'a> FileDialogBuilder<'a> {
    fn new(dialog: &'a mut FileDialog, mode: DialogMode) -> Self {
        Self {
            dialog,
            mode,
            title: None,
            filters: None,
            path: None,
            file_name: None,
            max_selection: 1,
            modal: false,
            flags: sys::ImGuiFileDialogFlags_None,
        }
    }

    /// Set the dialog title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(CString::new(title).unwrap());
        self
    }

    /// Set the file filters.
    ///
    /// Format: ".ext1,.ext2,.ext3" or "Description{.ext1,.ext2}" or ".*" for all files.
    pub fn filters(mut self, filters: &str) -> Self {
        self.filters = Some(CString::new(filters).unwrap());
        self
    }

    /// Set the initial directory path.
    pub fn path(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.path = Some(CString::new(path.as_ref().to_string_lossy().as_ref()).unwrap());
        self
    }

    /// Set the default file name (for save dialogs).
    pub fn file_name(mut self, name: &str) -> Self {
        self.file_name = Some(CString::new(name).unwrap());
        self
    }

    /// Allow multiple file selection.
    ///
    /// * `0` means infinite selection
    /// * `1` means single selection (default)
    /// * `n` means up to n files
    pub fn multi_select(mut self, max: i32) -> Self {
        self.max_selection = max;
        self
    }

    /// Make this a modal dialog.
    pub fn modal(mut self) -> Self {
        self.modal = true;
        self
    }

    /// Show confirmation dialog when overwriting files (for save dialogs).
    pub fn confirm_overwrite(mut self) -> Self {
        self.flags |= sys::ImGuiFileDialogFlags_ConfirmOverwrite;
        self
    }

    /// Open the dialog with the configured options.
    ///
    /// # Arguments
    /// * `key` - Unique key to identify this dialog instance
    pub fn build(self, key: &str) {
        let key_c = CString::new(key).unwrap();

        let default_title = match self.mode {
            DialogMode::OpenFile => CString::new("Open File").unwrap(),
            DialogMode::OpenDirectory => CString::new("Select Directory").unwrap(),
            DialogMode::SaveFile => CString::new("Save File").unwrap(),
        };

        let title = self.title.as_ref().unwrap_or(&default_title);

        // For directory mode, filters should be null
        let filters_ptr = match self.mode {
            DialogMode::OpenDirectory => std::ptr::null(),
            _ => {
                let default_filter = CString::new(".*").unwrap();
                self.filters
                    .as_ref()
                    .map(|f| f.as_ptr())
                    .unwrap_or(default_filter.as_ptr())
            }
        };

        let default_path = CString::new(".").unwrap();
        let path = self.path.as_ref().unwrap_or(&default_path);

        let default_filename = CString::new("").unwrap();
        let filename = self.file_name.as_ref().unwrap_or(&default_filename);

        unsafe {
            if self.modal {
                sys::IGFD_OpenModal(
                    self.dialog.ptr,
                    key_c.as_ptr(),
                    title.as_ptr(),
                    filters_ptr,
                    path.as_ptr(),
                    filename.as_ptr(),
                    self.max_selection,
                    std::ptr::null_mut(),
                    self.flags,
                );
            } else {
                sys::IGFD_OpenDialog(
                    self.dialog.ptr,
                    key_c.as_ptr(),
                    title.as_ptr(),
                    filters_ptr,
                    path.as_ptr(),
                    filename.as_ptr(),
                    self.max_selection,
                    std::ptr::null_mut(),
                    self.flags,
                );
            }
        }
    }
}

/// Collection of selected files from the dialog.
pub struct Selection {
    inner: sys::IGFD_Selection,
}

impl Selection {
    fn new(inner: sys::IGFD_Selection) -> Self {
        Self { inner }
    }

    /// Get the number of selected files.
    pub fn count(&self) -> usize {
        self.inner.count
    }

    /// Get an iterator over selected file paths.
    pub fn files(&self) -> impl Iterator<Item = PathBuf> + '_ {
        (0..self.inner.count).map(move |i| unsafe {
            let pair = &*self.inner.table.add(i);
            ptr_to_pathbuf(pair.filePathName)
        })
    }

    /// Get all selected file paths as a vector.
    pub fn into_vec(self) -> Vec<PathBuf> {
        self.files().collect()
    }
}

impl Drop for Selection {
    fn drop(&mut self) {
        unsafe { sys::IGFD_Selection_DestroyContent(&mut self.inner) }
    }
}

// ============================================================
// Bookmarks API (USE_BOOKMARK in v0.5.4)
// ============================================================
#[cfg(feature = "bookmark")]
impl FileDialog {
    /// Serialize bookmarks to a string for saving.
    pub fn serialize_bookmarks(&self) -> String {
        unsafe {
            let ptr = sys::IGFD_SerializeBookmarks(self.ptr);
            if ptr.is_null() {
                return String::new();
            }
            let s = ptr_to_string(ptr);
            libc::free(ptr as *mut _);
            s
        }
    }

    /// Deserialize bookmarks from a saved string.
    pub fn deserialize_bookmarks(&mut self, bookmarks: &str) {
        let bookmarks_c = CString::new(bookmarks).unwrap();
        unsafe { sys::IGFD_DeserializeBookmarks(self.ptr, bookmarks_c.as_ptr()) }
    }
}

// ============================================================
// Keyboard Navigation API
// ============================================================
#[cfg(feature = "exploration_by_keys")]
impl FileDialog {
    /// Set the flashing attenuation time for keyboard navigation highlight.
    pub fn set_flashing_attenuation(&mut self, seconds: f32) {
        unsafe { sys::IGFD_SetFlashingAttenuationInSeconds(self.ptr, seconds) }
    }
}

// ============================================================
// Helper functions
// ============================================================

unsafe fn ptr_to_string(ptr: *mut c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

unsafe fn ptr_to_pathbuf(ptr: *mut c_char) -> PathBuf {
    PathBuf::from(CStr::from_ptr(ptr).to_string_lossy().as_ref())
}
