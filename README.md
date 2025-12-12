# GlassUI

**GlassUI** is a futuristic, GPU-accelerated UI library for Rust, built on top of `winit` and `wgpu`. It provides **True Glassmorphism** with real-time background blurring, smooth animations, and a comprehensive widget system.

---

## ✨ Features

### Rendering

- **True Glassmorphism**: Dual-pass Compute Shader for real-time separable Gaussian blur
- **GPU Acceleration**: Fully powered by `wgpu` (Vulkan, Metal, DX12, WebGPU)
- **Text Rendering**: Dynamic font atlas using `ab_glyph`
- **Batched Rendering**: Efficient draw call batching with scissor rect support

### Layout System

| Widget | Description |
|--------|-------------|
| `Row` | Horizontal layout container |
| `Column` | Vertical layout container |
| `Stack` | Z-layered overlay container |
| `Align` | Alignment wrapper (Center, TopLeft, BottomLeft) |
| `Panel` | Container with glass background |
| `Spacer` | Flexible spacing element |
| `Grid` | CSS Grid-inspired multi-column layout |
| `Flex` | Flexbox-style layout with justify/align options |

### Interactive Widgets

| Widget | Description |
|--------|-------------|
| `Button` | Animated buttons with hover/press effects |
| `Slider` | Smooth draggable value slider |
| `Checkbox` | Toggleable state control |
| `Label` | Text display |
| `TextInput` | Editable text field with cursor |
| `Dropdown` | Expandable selection menu |
| `TabBar` | Tabbed content container |

### Advanced Features

| Widget | Description |
|--------|-------------|
| `Draggable` | Wrapper for movable elements (windows) |
| `Resizable` | Wrapper for resizable elements with handle |
| `ScrollArea` | Scrollable container with content clipping |
| `Tooltip` | Hover tooltip wrapper |
| `ContextMenuTrigger` | Right-click context menu wrapper |
| `Modal` | Dialog overlay with backdrop and animations |
| `DragSource` | Drag-and-drop source wrapper |
| `DropTarget` | Drag-and-drop target with callbacks |
| `Accessible` | Accessibility wrapper with ARIA-like labels |

### Theme System

| Theme | Description |
|-------|-------------|
| `Theme::cyberpunk()` | Neon cyan/magenta futuristic theme (default) |
| `Theme::dark()` | Modern dark mode with blue accents |
| `Theme::light()` | Clean light theme for accessibility |

### Interactivity

- **Focus Management**: Z-sorting with click-to-front behavior
- **Animations**: Smooth hover and press transitions
- **Keyboard Input**: Full text input support with backspace
- **Drag-and-Drop**: Inter-widget data transfer system
- **Accessibility**: Tab navigation and ARIA-like labeling


---

## 🚀 Getting Started

### Prerequisites

- Rust stable toolchain (1.70+)
- GPU with Vulkan/Metal/DX12 support

### Running the Demo

```sh
cd glassui
cargo run --release
```

The demo showcases a "Glass OS" dashboard with:

- Draggable windows
- Resizable panels
- Scrollable log viewer
- Context menus
- Interactive controls

---

## 📖 Usage Example

```rust
use glassui::GlassContext;
use glassui::widget::{
    Panel, Column, Row, Button, Label, Slider, Checkbox,
    Stack, Align, Alignment, Draggable, Resizable,
    TextInput, ScrollArea, Tooltip, ContextMenuTrigger, MenuItem
};

// Create UI hierarchy
let content = Column::new()
    .add_child(Box::new(Label::new("GlassUI Demo")))
    .add_child(Box::new(Slider::new(0.5)))
    .add_child(Box::new(Checkbox::new("Enable Feature", true)))
    .add_child(Box::new(TextInput::new("Enter text...")))
    .add_child(Box::new(Row::new()
        .add_child(Box::new(Tooltip::new(
            Box::new(Button::new("Save")),
            "Save changes"
        )))
        .add_child(Box::new(Button::new("Cancel")))
    ));

let window = Panel::new(Box::new(content))
    .with_color(Vec4::new(0.1, 0.1, 0.15, 0.4))
    .with_fill(true);

// Make it draggable and resizable
let interactive_window = Draggable::new(Box::new(
    Resizable::new(Box::new(window), Vec2::new(400.0, 300.0))
));

// Add context menu
let with_menu = ContextMenuTrigger::new(
    Box::new(interactive_window),
    vec![
        MenuItem::new("Refresh"),
        MenuItem::new("Close"),
    ]
);

let mut root = Stack::new()
    .add_child(Box::new(Align::new(Alignment::Center, Box::new(with_menu))));

// In event loop:
context.update(0.016);
root.update(0.016);
root.layout(Vec2::ZERO, Vec2::new(width, height));
context.render(&mut root);
```

---

## 🏗️ Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                    Scene Pass                           │
│  Renders background (procedural grid) to offscreen tex  │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                    Blur Pass                            │
│  Compute shader: Horizontal blur → Vertical blur        │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                  Composite Pass                         │
│  Draw background → Batched UI widgets → Text overlay    │
│  (Glass panels sample from blurred texture)             │
└─────────────────────────────────────────────────────────┘
```

### Rendering Pipeline

1. **Scene Pass**: Renders animated background to `Rgba8Unorm` texture
2. **Blur Pass**: Two-pass separable Gaussian blur via Compute Shader
3. **Composite Pass**:
   - Draw sharp background
   - Iterate render batches with scissor rects
   - Draw glass instances sampling blurred texture
   - Draw text overlay
   - Draw tooltips (deferred)

---

## 📁 Project Structure

```text
glassui/
├── src/
│   ├── lib.rs          # Library exports and GlassContext
│   ├── main.rs         # Demo application
│   ├── renderer.rs     # wgpu rendering engine
│   ├── text.rs         # Font atlas and text rendering
│   ├── widget.rs       # All widget implementations
│   └── shaders/
│       ├── bg.wgsl     # Background shader
│       ├── blur.wgsl   # Gaussian blur compute shader
│       ├── glass.wgsl  # Glass panel shader
│       └── text.wgsl   # Text rendering shader
├── Cargo.toml
└── README.md
```

---

## 🎨 Widget Reference

### Layout Widgets

```rust
// Vertical layout
Column::new()
    .add_child(Box::new(widget1))
    .add_child(Box::new(widget2))

// Horizontal layout
Row::new()
    .add_child(Box::new(widget1))
    .add_child(Box::new(widget2))

// Overlay stack (last child on top)
Stack::new()
    .add_child(Box::new(background))
    .add_child(Box::new(foreground))

// Alignment wrapper
Align::new(Alignment::Center, Box::new(child))
```

### Input Widgets

```rust
// Button with custom size (200x50 default)
Button::new("Click Me")

// Slider (0.0 to 1.0)
Slider::new(0.5)

// Checkbox with label
Checkbox::new("Enable", true)

// Text input
TextInput::new("Placeholder...")
```

### Container Widgets

```rust
// Glass panel
Panel::new(Box::new(content))
    .with_color(Vec4::new(r, g, b, a))
    .with_fill(true)

// Scrollable area
ScrollArea::new(Box::new(tall_content))

// Draggable wrapper
Draggable::new(Box::new(window))

// Resizable wrapper
Resizable::new(Box::new(panel), Vec2::new(400.0, 300.0))
```

### Overlay Widgets

```rust
// Tooltip (shows after 0.5s hover)
Tooltip::new(Box::new(button), "Helpful text")

// Context menu (right-click)
ContextMenuTrigger::new(
    Box::new(target),
    vec![
        MenuItem::new("Action 1"),
        MenuItem::new("Action 2"),
    ]
)
```

---

## 📜 License

MIT

---

## 🙏 Acknowledgments

- [wgpu](https://wgpu.rs/) - Modern GPU API
- [winit](https://github.com/rust-windowing/winit) - Window management
- [ab_glyph](https://github.com/alexheretic/ab-glyph) - Font rendering
- [glam](https://github.com/bitshifter/glam-rs) - Math library
