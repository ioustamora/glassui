use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use glassui::GlassContext;
use glassui::widget::{Panel, Button, Label, Slider, Checkbox, Column, Row, Widget, Stack, Align, Alignment, Spacer, Draggable, Resizable, TextInput, ScrollArea, Tooltip, ContextMenuTrigger, MenuItem, Dropdown, TabBar};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("GlassUIOS 2077").with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0)).build(&event_loop).unwrap();
    
    let mut context = pollster::block_on(GlassContext::new(&window));
    
    // --- Window Content ---
    let window_content = Column::new()
        .add_child(Box::new(Label::new("Glass Control")))
        .add_child(Box::new(Spacer::new(glassui::Vec2::new(0.0, 20.0))))
        .add_child(Box::new(Label::new("System Status: ONLINE")))
        .add_child(Box::new(Slider::new(0.7)))
        .add_child(Box::new(Checkbox::new("Neon Mode", true)))
        .add_child(Box::new(Checkbox::new("Turbo", false)))
        .add_child(Box::new(TextInput::new("Enter command...")))
        .add_child(Box::new(Dropdown::new(vec![
            "Low".to_string(),
            "Medium".to_string(),
            "High".to_string(),
            "Ultra".to_string(),
        ])))
        .add_child(Box::new(Spacer::new(glassui::Vec2::new(0.0, 20.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(Button::new("Apply")))
            .add_child(Box::new(Spacer::new(glassui::Vec2::new(10.0, 0.0))))
            .add_child(Box::new(Button::new("Close")))
        ));
        
    let window_panel = Panel::new(Box::new(window_content))
        .with_color(glassui::Vec4::new(0.1, 0.1, 0.15, 0.4))
        .with_fill(true); // Fill the Resizable container

    // --- Taskbar ---
    let taskbar_content = Row::new()
        .add_child(Box::new(Button::new("Start")))
        .add_child(Box::new(Spacer::new(glassui::Vec2::new(20.0, 0.0))))
        .add_child(Box::new(Button::new("Browser")))
        .add_child(Box::new(Button::new("Terminal")))
        .add_child(Box::new(Tooltip::new(Box::new(Button::new("Settings")), "Open System Settings")))
        .add_child(Box::new(Spacer::new(glassui::Vec2::new(50.0, 0.0))))
        .add_child(Box::new(Label::new("20:77 PM")));
        
    let taskbar_panel = Panel::new(Box::new(taskbar_content))
        .with_color(glassui::Vec4::new(0.0, 0.0, 0.0, 0.6));

    // --- Desktop Icons ---
    let desktop_icons = Column::new()
        .add_child(Box::new(Button::new("My Computer")))
        .add_child(Box::new(Button::new("Recycle Bin")))
        .add_child(Box::new(Button::new("Network")));

    let desktop_panel = Panel::new(Box::new(desktop_icons))
        .with_color(glassui::Vec4::new(0.0, 0.0, 0.0, 0.0)); // Transparent container

    let desktop_with_menu = ContextMenuTrigger::new(
        Box::new(desktop_panel),
        vec![
            MenuItem::new("Refresh"),
            MenuItem::new("New Folder"),
            MenuItem::new("Properties"),
        ]
    );

    // --- Logs Window (Scrollable) ---
    let mut logs = Column::new();
    for i in 0..20 {
        logs = logs.add_child(Box::new(Label::new(&format!("System Log Entry #{}", i))));
    }
    let logs_panel = Panel::new(Box::new(logs)).with_color(glassui::Vec4::new(0.1, 0.1, 0.15, 0.8)).with_fill(true);
    let logs_scroll = ScrollArea::new(Box::new(logs_panel));
    let logs_window = Resizable::new(Box::new(logs_scroll), glassui::Vec2::new(300.0, 200.0));

    // --- Settings Window (TabBar) ---
    let tab_content_1 = Column::new()
        .add_child(Box::new(Label::new("Display Settings")))
        .add_child(Box::new(Slider::new(0.8)))
        .add_child(Box::new(Checkbox::new("Fullscreen", false)));
    
    let tab_content_2 = Column::new()
        .add_child(Box::new(Label::new("Audio Settings")))
        .add_child(Box::new(Slider::new(0.5)))
        .add_child(Box::new(Checkbox::new("Mute", false)));
    
    let tab_content_3 = Column::new()
        .add_child(Box::new(Label::new("Network Settings")))
        .add_child(Box::new(TextInput::new("Server IP...")));

    let settings_tabs = TabBar::new()
        .add_tab("Display", Box::new(tab_content_1))
        .add_tab("Audio", Box::new(tab_content_2))
        .add_tab("Network", Box::new(tab_content_3));
    
    let settings_window = Resizable::new(Box::new(settings_tabs), glassui::Vec2::new(400.0, 250.0));

    // --- Root Stack ---
    let mut root = Stack::new()
        .add_child(Box::new(Align::new(Alignment::TopLeft, Box::new(Draggable::new(Box::new(desktop_with_menu))))))
        // Control Window
        .add_child(Box::new(Align::new(Alignment::Center, Box::new(Draggable::new(Box::new(Resizable::new(Box::new(window_panel), glassui::Vec2::new(400.0, 400.0))))))))
        // Logs Window
        .add_child(Box::new(Align::new(Alignment::BottomLeft, Box::new(Spacer::new(glassui::Vec2::new(0.0, 400.0)))))) // Hack to offset position safely? No, Draggable helps.
        // Actually, let's just use Draggable random position? 
        // Our Align wrapper forces position.
        // Let's just put it Center but Draggable will let user move it.
        // Overlapping windows is fine (Z-sort works).
        .add_child(Box::new(Align::new(Alignment::Center, Box::new(Draggable::new(Box::new(logs_window))))))
        // Settings Window
        .add_child(Box::new(Align::new(Alignment::Center, Box::new(Draggable::new(Box::new(settings_window))))))
        
        .add_child(Box::new(Align::new(Alignment::BottomLeft, Box::new(taskbar_panel))));

    let mut cursor_pos = glassui::Vec2::ZERO;

    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                target.exit();
            }
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                context.resize(physical_size.width, physical_size.height);
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                cursor_pos = glassui::Vec2::new(position.x as f32, position.y as f32);
                root.handle_event(&Event::WindowEvent { window_id: unsafe { winit::window::WindowId::dummy() }, event: WindowEvent::CursorMoved { device_id: unsafe { winit::event::DeviceId::dummy() }, position } }, cursor_pos); 
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                context.update(0.016);
                root.update(0.016);
                root.layout(glassui::Vec2::ZERO, glassui::Vec2::new(context.width as f32, context.height as f32));
                context.render(&mut root);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {
                // Pass other events (like MouseInput) with the cached cursor_pos
                root.handle_event(&event, cursor_pos);
            }
        }
    }).unwrap();
}

