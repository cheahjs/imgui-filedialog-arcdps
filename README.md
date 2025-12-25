# ImGuiFileDialog Bindings for arcdps-imgui

Rust bindings to [ImGuiFileDialog](https://github.com/aiekick/ImGuiFileDialog) v0.5.4, a file selection dialog for Dear ImGui, designed for use with [arcdps-imgui](https://crates.io/crates/arcdps-imgui).

**Note**: This crate targets ImGuiFileDialog v0.5.4 which is compatible with **ImGui 1.80** (as used by arcdps-imgui).

## Features

- **Open/Save dialogs** - Browse and select files or directories
- **Multi-select** - Select multiple files at once  
- **File filters** - Filter by extension (e.g., `.txt,.md,.rs`)
- **Extension styles** - Custom colors and icons for file types
- **Bookmarks** - Save and restore favorite directories
- **Keyboard navigation** - Navigate with arrow keys

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
imgui-filedialog = { git = "https://github.com/jscheah/imgui-filedialog-arcdps" }
```

### Quick Start

```rust
use imgui_filedialog::FileDialog;

// Create a dialog instance (store in your app state)
let mut dialog = FileDialog::new();

// Open a file selection dialog when a button is clicked
if ui.button("Open File") {
    dialog.open_file()
        .title("Select a File")
        .filters(".txt,.md,.rs")
        .path(".")
        .multi_select(0)  // 0 = unlimited
        .build("choose_file");
}

// Render the dialog every frame
if dialog.display("choose_file", [400.0, 300.0], [800.0, 600.0]) {
    if dialog.is_ok() {
        if let Some(selection) = dialog.selection() {
            for path in selection.files() {
                println!("Selected: {:?}", path);
            }
        }
    }
    dialog.close();
}
```

### Save Dialog

```rust
dialog.save_file()
    .title("Save As")
    .filters(".txt")
    .path(".")
    .file_name("untitled.txt")
    .confirm_overwrite()
    .build("save_file");
```

### Directory Selection

```rust
dialog.open_directory()
    .title("Select Folder")
    .path(".")
    .build("choose_dir");
```

### Modal Dialogs

```rust
dialog.open_file()
    .title("Select File")
    .modal()  // Makes it a modal dialog
    .build("modal_file");
```

## Optional Features

| Feature | Default | Description |
|---------|---------|-------------|
| `bookmark` | ✓ | Bookmarks/favorites panel |
| `exploration_by_keys` | ✓ | Keyboard navigation |

Disable default features:
```toml
[dependencies]
imgui-filedialog = { git = "...", default-features = false }
```

## Crate Structure

- **`imgui-filedialog`** - Safe, idiomatic Rust API (recommended)
- **`imgui-filedialog-sys`** - Raw FFI bindings to the C API

## Git Submodule Setup

This crate uses ImGuiFileDialog as a git submodule. After cloning, run:

```bash
git submodule update --init --recursive
```

## License

MIT License - same as ImGuiFileDialog.
