# GlassUI

> **⚠️ Active Development** — GlassUI is under heavy development. APIs may change frequently.

**GlassUI** is a futuristic, GPU-accelerated **Reactive Application Development (RAD)** framework for Rust, built on `winit` and `wgpu`. It provides **True Glassmorphism** with real-time background blurring, spring-based animations, gesture recognition, and a comprehensive modular widget system.

---

## ✨ Features

### 🎨 Rendering Engine

- **True Glassmorphism**: Dual-pass Compute Shader for real-time separable Gaussian blur
- **GPU Acceleration**: Fully powered by `wgpu` (Vulkan, Metal, DX12, WebGPU)
- **Text Rendering**: Dynamic font atlas using `ab_glyph`
- **Batched Rendering**: Efficient draw call batching with scissor rect support

### 📐 Layout System

| Widget | Description |
|--------|-------------|
| `Row` | Horizontal layout container |
| `Column` | Vertical layout container |
| `Stack` | Z-layered overlay container |
| `Align` | Alignment wrapper (Center, TopLeft, BottomLeft, etc.) |
| `Panel` | Container with glass background |
| `Spacer` | Flexible spacing element |
| `Grid` | CSS Grid-inspired multi-column layout |
| `Flex` | Flexbox-style layout with justify/align options |

### 🎛️ Control Widgets

| Widget | Description |
|--------|-------------|
| `Button` | Animated buttons with hover/press effects |
| `Slider` | Smooth draggable value slider |
| `Checkbox` | Toggleable state control |
| `Toggle` | iOS-style toggle switch |
| `RadioGroup` | Mutually exclusive option selection |
| `NumberInput` | Numeric input with increment/decrement |
| `ProgressBar` | Visual progress indicator |

### ⌨️ Input Widgets

| Widget | Description |
|--------|-------------|
| `Label` | Text display |
| `TextInput` | Editable text field with cursor and clipboard |
| `Dropdown` | Expandable selection menu |
| `DatePicker` | Calendar-based date selection |
| `RichTextEditor` | Multi-style text editing |

### 📊 Data Visualization

| Widget | Description |
|--------|-------------|
| `Table` | Sortable data table with columns |
| `ListView` | Virtualized scrollable list |
| `TreeView` | Hierarchical expandable tree |
| `LineChart` | Line graph visualization |
| `BarChart` | Horizontal/vertical bar charts |
| `PieChart` | Circular data representation |
| `Sparkline` | Compact inline charts |

### 🖼️ Media Widgets

| Widget | Description |
|--------|-------------|
| `Image` | Image display with multiple fit modes |
| `Icon` | Scalable icon rendering |
| `RichText` | Styled text with spans |
| `VideoPlayer` | Video playback with controls and seek bar |

### 📦 Container Widgets

| Widget | Description |
|--------|-------------|
| `ScrollArea` | Scrollable container with content clipping |
| `TabBar` | Tabbed content container |
| `Modal` | Dialog overlay with backdrop and animations |

### 🔧 Advanced Widgets

| Widget | Description |
|--------|-------------|
| `Draggable` | Wrapper for movable elements (windows) |
| `Resizable` | Wrapper for resizable elements with handle |
| `Tooltip` | Hover tooltip wrapper |
| `ContextMenuTrigger` | Right-click context menu wrapper |
| `HeroScope` | Shared element transition wrapper |

---

## 🎬 Animation System

GlassUI features a powerful animation engine:

- **Tween Animations**: Property interpolation with easing curves
- **Spring Animations**: Physics-based spring dynamics
- **Animation Sequences**: Chain multiple animations
- **Animation Groups**: Run animations in parallel
- **Delayed Animations**: Schedule animations for later
- **Built-in Curves**: Linear, EaseIn, EaseOut, EaseInOut, Bounce, Elastic, etc.

```rust
use glassui::{Tween, Curve, SpringAnimation};

// Tween animation
let fade_in = Tween::new(0.0, 1.0, 0.3, Curve::EaseOut);

// Spring animation
let spring = SpringAnimation::new(0.0, 100.0, 300.0, 0.7); // stiffness, damping
```

---

## 🦸 Hero Transitions

Smooth shared element transitions between views:

- **HeroScope**: Widget wrapper that marks elements for transitions
- **HeroController**: Manages and coordinates active transitions
- **HeroFlight**: Active transition with position/size/opacity interpolation

```rust
use glassui::{HeroScope, HeroController, HeroId, HeroRect};

// Mark elements with matching IDs
let thumbnail = HeroScope::new("avatar", Box::new(Image::new(...)));

// Trigger transition
let mut controller = HeroController::new();
controller.start_flight(
    HeroId::new("avatar"),
    source_bounds,
    destination_bounds,
);
```

---

## 👆 Gesture Recognition

Full touch and pointer gesture support:

- **Tap / Double Tap**: Click detection
- **Long Press**: Hold detection with duration
- **Pan**: Drag with velocity tracking
- **Pinch**: Two-finger scale gesture
- **Rotation**: Two-finger rotate gesture

---

## 🎨 Theme System

| Theme | Description |
|-------|-------------|
| `Theme::cyberpunk()` | Neon cyan/magenta futuristic theme (default) |
| `Theme::dark()` | Modern dark mode with blue accents |
| `Theme::light()` | Clean light theme for accessibility |

### CSS-like Styling

```rust
use glassui::{WidgetStyle, ButtonVariant, SizeVariant};

let style = WidgetStyle::new()
    .variant(ButtonVariant::Primary)
    .size(SizeVariant::Large);
```

---

## ⌨️ Framework Features

- **Focus Management**: Tab navigation and Z-sorting with click-to-front
- **Clipboard Support**: Cross-platform copy/paste via `arboard`
- **Undo/Redo**: Command pattern with history
- **Accessibility**: ARIA-like labeling for screen readers
- **Constraint Layout**: Flexible box constraints system

---

## 🚀 Getting Started

### Prerequisites

- Rust stable toolchain (1.70+)
- GPU with Vulkan/Metal/DX12 support

### Running the Demo

```sh
cargo run --release
```

The demo showcases a "Glass OS" dashboard with draggable windows, resizable panels, scrollable content, context menus, and interactive controls.

---

## 📖 Usage Example

```rust
use glassui::GlassContext;
use glassui::widgets::{
    Panel, Column, Row, Button, Label, Slider, Checkbox,
    Stack, Align, Alignment, Draggable, Resizable,
    TextInput, ScrollArea, Tooltip, ContextMenuTrigger, MenuItem,
    ProgressBar, Toggle, LineChart, DataSeries, DataPoint,
};

// Create UI with new widgets
let content = Column::new()
    .add_child(Box::new(Label::new("GlassUI RAD Framework")))
    .add_child(Box::new(ProgressBar::new(0.75)))
    .add_child(Box::new(Toggle::new("Dark Mode", true)))
    .add_child(Box::new(Slider::new(0.5)))
    .add_child(Box::new(TextInput::new("Enter text...")))
    .add_child(Box::new(Row::new()
        .add_child(Box::new(Button::new("Save")))
        .add_child(Box::new(Button::new("Cancel")))
    ));

let window = Panel::new(Box::new(content))
    .with_color(Vec4::new(0.1, 0.1, 0.15, 0.4))
    .with_fill(true);

let interactive_window = Draggable::new(Box::new(
    Resizable::new(Box::new(window), Vec2::new(400.0, 300.0))
));
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

---

## 📁 Project Structure

```text
glassui/
├── src/
│   ├── lib.rs           # Library exports and GlassContext
│   ├── main.rs          # Demo application
│   ├── renderer.rs      # wgpu rendering engine
│   ├── text.rs          # Font atlas and text rendering
│   ├── animation.rs     # Animation system (tweens, springs)
│   ├── gestures.rs      # Gesture recognition
│   ├── focus.rs         # Focus management
│   ├── clipboard.rs     # Clipboard support
│   ├── accessibility.rs # Screen reader support
│   ├── commands.rs      # Undo/redo command pattern
│   ├── style.rs         # CSS-like styling
│   ├── layout.rs        # Constraint-based layout
│   ├── state.rs         # State management
│   ├── property.rs      # Reactive properties
│   ├── macros.rs        # Declarative widget macros
│   ├── widget.rs        # Legacy widget module
│   ├── widgets/         # Modular widget system
│   │   ├── core.rs      # Theme, Widget trait
│   │   ├── layout.rs    # Layout containers
│   │   ├── controls.rs  # Buttons, sliders, etc.
│   │   ├── input.rs     # Text input, dropdown, date picker
│   │   ├── containers.rs# ScrollArea, TabBar
│   │   ├── overlays.rs  # Tooltip, Modal, ContextMenu
│   │   ├── advanced.rs  # Draggable, Resizable
│   │   ├── premium.rs   # ProgressBar, Toggle, RadioGroup
│   │   ├── data.rs      # Table, ListView, TreeView
│   │   ├── charts.rs    # LineChart, BarChart, PieChart
│   │   ├── media.rs     # Image, Icon
│   │   └── richtext.rs  # RichText, RichTextEditor
│   └── shaders/
│       ├── bg.wgsl      # Background shader
│       ├── blur.wgsl    # Gaussian blur compute shader
│       ├── composite.wgsl # Composite pass shader
│       ├── glass.wgsl   # Glass panel shader
│       └── text.wgsl    # Text rendering shader
├── Cargo.toml
└── README.md
```

---

## 📜 License

MIT

---

## 💖 Support Development

If you find GlassUI useful, consider supporting its development:

| Network | Wallet Address |
|---------|----------------|
| **TRON (TRX/USDT/USDD)** | `TUkJz3XH25BFQx2Ur28jMWYx63EEQyGYVu` |

---

## 🙏 Acknowledgments

- [wgpu](https://wgpu.rs/) — Modern GPU API
- [winit](https://github.com/rust-windowing/winit) — Window management
- [ab_glyph](https://github.com/alexheretic/ab-glyph) — Font rendering
- [glam](https://github.com/bitshifter/glam-rs) — Math library
- [arboard](https://github.com/1Password/arboard) — Clipboard support
