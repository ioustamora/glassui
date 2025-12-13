//! GlassUI v2 Demo
//!
//! Showcasing all widgets including:
//! - StatusBar, ToastContainer, AgentCard
//! - TabView, FileTree, DataTable
//! - AnimatedProgressBar, CircularGauge, MetricDisplay
//! - CommandPalette, Timeline
//! - Charts, Controls, and more

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use glassui::GlassContext;
use glassui::widgets::{
    Widget, Panel, Button, Label, Slider, Checkbox, Column, Row, Stack, 
    Align, Alignment, Spacer, Draggable, Resizable,
    TextInput, Dropdown, Toggle, ProgressBar, NumberInput, RadioGroup,
    ScrollArea, TabBar, Tooltip,
    Table, TableColumn, TableRow, ListView, ListItem,
    LineChart, BarChart, PieChart, Sparkline,
    RichText, RichTextEditor,
    // V2 Widgets
    StatusBar, ToastContainer, AgentCard, Toast, ToastType,
    CommandPalette, Command,
    Timeline, TimelineEntry, TimelineEntryType,
    AnimatedProgressBar, CircularGauge, MetricDisplay, MetricTrend,
    Tab, TabView,
    FileNode, FileTree,
    GridColumn, GridRow, DataTable, CellValue,
    ControllablePanel,
};
use glassui::ai::AgentState;
use glassui::shortcuts::{ShortcutManager, Shortcut, ShortcutKey};
use glassui::{Vec2, Vec4};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("GlassUI v2 Dashboard Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0))
        .build(&event_loop).unwrap();
    
    let mut context = pollster::block_on(GlassContext::new(&window));
    
    // =========================================================================
    // KEYBOARD SHORTCUTS
    // =========================================================================
    
    let mut shortcuts = ShortcutManager::new();
    shortcuts.register_dashboard_shortcuts();
    
    // =========================================================================
    // STATUS BAR (TOP)
    // =========================================================================
    
    let mut status_bar = StatusBar::dashboard_default();
    
    // =========================================================================
    // TOAST CONTAINER (TOP RIGHT)
    // =========================================================================
    
    let mut toasts = ToastContainer::new().position_top_right(1600.0, 16.0);
    // Add initial toast
    toasts.info("Welcome", "GlassUI v2 Dashboard loaded");
    
    // =========================================================================
    // COMMAND PALETTE (OVERLAY)
    // =========================================================================
    
    let mut command_palette = CommandPalette::new()
        .with_dashboard_commands()
        .center_on_screen(Vec2::new(1600.0, 900.0));
    
    // =========================================================================
    // AI AGENT CARD
    // =========================================================================
    
    let mut agent_card = AgentCard::new("Assistant", "phi3");
    agent_card.set_state(AgentState::Idle);
    
    // =========================================================================
    // TAB VIEW (NEW V2)
    // =========================================================================
    
    let mut tab_view = TabView::new();
    tab_view.add_tab(Tab::new("dashboard", "Dashboard").with_icon("📊"));
    tab_view.add_tab(Tab::new("analytics", "Analytics").with_icon("📈"));
    tab_view.add_tab(Tab::new("settings", "Settings").with_icon("⚙").closeable());
    
    // =========================================================================
    // FILE TREE (NEW V2)
    // =========================================================================
    
    let file_tree = FileTree::sample_file_tree();
    
    // =========================================================================
    // DATA TABLE (NEW V2)
    // =========================================================================
    
    let data_table = DataTable::sample();
    
    // =========================================================================
    // GAUGES (NEW V2)
    // =========================================================================
    
    let mut cpu_gauge = CircularGauge::new(0.45)
        .with_label("CPU")
        .with_color(Vec4::new(0.4, 0.7, 1.0, 1.0));
    
    let mut mem_gauge = CircularGauge::new(0.62)
        .with_label("Memory")
        .with_color(Vec4::new(0.5, 0.9, 0.5, 1.0));
    
    let mut progress = AnimatedProgressBar::new(0.72);
    
    let cpu_metric = MetricDisplay::new("CPU Usage", "45%")
        .with_trend(MetricTrend::Up, "+5%")
        .with_sparkline_data(vec![0.3, 0.4, 0.35, 0.45, 0.5, 0.42, 0.48]);
    
    // =========================================================================
    // TIMELINE (NEW V2)
    // =========================================================================
    
    let timeline = Timeline::sample();
    
    // =========================================================================
    // ORIGINAL CHARTS PANEL
    // =========================================================================
    
    let line_chart = LineChart::new()
        .with_data("Revenue", &[12.0, 28.0, 19.0, 38.0, 32.0, 45.0, 40.0])
        .with_size(260.0, 150.0);
    
    let bar_chart = BarChart::new()
        .with_data("Sales", &[80.0, 120.0, 90.0, 140.0, 100.0])
        .with_size(260.0, 150.0);
    
    let charts_content = Column::new()
        .add_child(Box::new(Label::new("📊 Charts")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(line_chart))
            .add_child(Box::new(Spacer::new(Vec2::new(8.0, 0.0))))
            .add_child(Box::new(bar_chart))
        ));
    
    let charts_panel = Panel::new(Box::new(charts_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // ORIGINAL CONTROLS PANEL
    // =========================================================================
    
    let orig_progress = ProgressBar::new(0.72).with_color(Vec4::new(0.3, 0.85, 0.5, 1.0));
    let toggle = Toggle::new("Enable Notifications", true);
    let number = NumberInput::new(50.0).with_range(0.0, 100.0).with_step(10.0);
    let radio = RadioGroup::new(vec![
        "Low".to_string(), "Medium".to_string(), "High".to_string()
    ]).with_selected(1);
    
    let controls_content = Column::new()
        .add_child(Box::new(Label::new("🎛 Controls")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(orig_progress))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(toggle))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(number))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(radio));
    
    let controls_panel = Panel::new(Box::new(controls_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // INPUT PANEL
    // =========================================================================
    
    let text_input = TextInput::new("Search...");
    let slider = Slider::new(0.65);
    let checkbox = Checkbox::new("Dark Mode", true);
    let dropdown = Dropdown::new(vec![
        "Option A".to_string(), "Option B".to_string(), "Option C".to_string()
    ]);
    
    let inputs_content = Column::new()
        .add_child(Box::new(Label::new("📝 Inputs")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(text_input))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(slider))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(checkbox))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(dropdown))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 10.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(Button::new("OK")))
            .add_child(Box::new(Spacer::new(Vec2::new(8.0, 0.0))))
            .add_child(Box::new(Button::new("Cancel")))
        ));
    
    let inputs_panel = Panel::new(Box::new(inputs_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // V2 PANELS - Using new widgets
    // =========================================================================
    
    // Gauges panel
    let gauges_content = Column::new()
        .add_child(Box::new(Label::new("📊 System Metrics")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(cpu_metric));
    
    let gauges_panel = Panel::new(Box::new(gauges_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // Agent panel
    let agent_content = Column::new()
        .add_child(Box::new(Label::new("🤖 AI Agent")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(agent_card));
    
    let agent_panel = Panel::new(Box::new(agent_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // Timeline panel
    let timeline_content = Column::new()
        .add_child(Box::new(Label::new("📅 Activity")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(timeline));
    
    let timeline_panel = Panel::new(Box::new(timeline_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // File tree panel
    let tree_content = Column::new()
        .add_child(Box::new(Label::new("📁 Files")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(file_tree));
    
    let tree_panel = Panel::new(Box::new(tree_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // Data table panel
    let table_content = Column::new()
        .add_child(Box::new(Label::new("📋 Services")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(data_table));
    
    let table_panel = Panel::new(Box::new(table_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // HEADER
    // =========================================================================
    
    let header = Panel::new(Box::new(Row::new()
        .add_child(Box::new(Label::new("GlassUI v2 Dashboard")))
        .add_child(Box::new(Spacer::new(Vec2::new(40.0, 0.0))))
        .add_child(Box::new(Label::new("Ctrl+K for commands | R=Resize M=Move C=Color")))
    ))
    .with_color(Vec4::new(0.04, 0.04, 0.08, 0.95));
    
    // =========================================================================
    // ROOT LAYOUT - All panels draggable
    // =========================================================================
    
    let mut root = Stack::new()
        // Header at top
        .add_child(Box::new(Align::new(Alignment::TopLeft, Box::new(header))))
        // Charts - top left
        .add_child(Box::new(Align::new(Alignment::TopLeft,
            Box::new(Draggable::new(Box::new(Resizable::new(
                Box::new(charts_panel), Vec2::new(560.0, 220.0))))))))
        // Controls - bottom left  
        .add_child(Box::new(Align::new(Alignment::BottomLeft,
            Box::new(Draggable::new(Box::new(controls_panel))))))
        // Inputs - center
        .add_child(Box::new(Align::new(Alignment::Center,
            Box::new(Draggable::new(Box::new(inputs_panel))))))
        // Gauges - top right (offset from agent)
        .add_child(Box::new(Align::new(Alignment::TopRight,
            Box::new(Draggable::new(Box::new(gauges_panel))))))
        // Agent - top right
        .add_child(Box::new(Align::new(Alignment::TopRight,
            Box::new(Draggable::new(Box::new(agent_panel))))))
        // Timeline - center right area
        .add_child(Box::new(Align::new(Alignment::Center,
            Box::new(Draggable::new(Box::new(timeline_panel))))))
        // File tree - bottom left (offset from controls)
        .add_child(Box::new(Align::new(Alignment::BottomLeft,
            Box::new(Draggable::new(Box::new(tree_panel))))))
        // Data table - bottom right
        .add_child(Box::new(Align::new(Alignment::BottomRight,
            Box::new(Draggable::new(Box::new(Resizable::new(
                Box::new(table_panel), Vec2::new(480.0, 200.0))))))));
    
    let mut cursor_pos = Vec2::ZERO;
    let mut command_palette_visible = false;

    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);

        // Handle keyboard shortcuts
        if let Some(action) = shortcuts.handle_event(&event) {
            match action.as_str() {
                "command_palette" => {
                    command_palette_visible = !command_palette_visible;
                    if command_palette_visible {
                        command_palette.show();
                    } else {
                        command_palette.hide();
                    }
                },
                "save_workspace" => {
                    toasts.success("Saved", "Workspace saved successfully!");
                },
                "new_panel" => {
                    toasts.info("Panel", "New panel created");
                },
                "deselect" => {
                    command_palette_visible = false;
                    command_palette.hide();
                },
                _ => {}
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                target.exit();
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                context.resize(size.width, size.height);
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                cursor_pos = Vec2::new(position.x as f32, position.y as f32);
                root.handle_event(&Event::WindowEvent { 
                    window_id: unsafe { winit::window::WindowId::dummy() }, 
                    event: WindowEvent::CursorMoved { 
                        device_id: unsafe { winit::event::DeviceId::dummy() }, 
                        position 
                    } 
                }, cursor_pos);
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                // Update
                context.update(0.016);
                root.update(0.016);
                toasts.update(0.016);
                status_bar.update(0.016);
                
                // Update status bar metrics (simulated from time)
                static mut FRAME: u32 = 0;
                unsafe { FRAME += 1; }
                let elapsed = unsafe { FRAME } as f32 * 0.016;
                let cpu = (40.0 + 30.0 * (elapsed * 0.5).sin()) as i32;
                let mem = (50.0 + 20.0 * (elapsed * 0.3).cos()) as i32;
                status_bar.update_item("cpu", &format!("{}%", cpu));
                status_bar.update_item("mem", &format!("{}%", mem));
                
                // Layout
                root.layout(Vec2::ZERO, Vec2::new(context.width as f32, context.height as f32));
                
                // Render
                context.render(&mut root);
                
                // Render overlays (status bar, toasts, command palette)
                // Note: In a real app, these would be rendered as part of the context
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {
                root.handle_event(&event, cursor_pos);
                
                // Handle command palette events
                if command_palette_visible {
                    command_palette.handle_event(&event, cursor_pos);
                }
            }
        }
    }).unwrap();
}
