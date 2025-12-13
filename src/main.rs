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
};
use glassui::{Vec2, Vec4};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("GlassUI Framework Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1400.0, 850.0))
        .build(&event_loop).unwrap();
    
    let mut context = pollster::block_on(GlassContext::new(&window));
    
    // =========================================================================
    // CHARTS PANEL
    // =========================================================================
    
    let line_chart = LineChart::new()
        .with_data("Revenue", &[12.0, 28.0, 19.0, 38.0, 32.0, 45.0, 40.0])
        .with_size(260.0, 150.0);
    
    let bar_chart = BarChart::new()
        .with_data("Sales", &[80.0, 120.0, 90.0, 140.0, 100.0])
        .with_size(260.0, 150.0);
    
    let pie_chart = PieChart::new()
        .with_values(&[35.0, 28.0, 22.0, 15.0])
        .donut(0.45)
        .with_size(120.0);
    
    let sparkline = Sparkline::new(vec![3.0, 7.0, 4.0, 9.0, 5.0, 8.0, 6.0])
        .with_size(90.0, 28.0)
        .with_color(Vec4::new(0.4, 0.9, 0.5, 1.0));
    
    let charts_content = Column::new()
        .add_child(Box::new(Label::new("Charts")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(line_chart))
            .add_child(Box::new(Spacer::new(Vec2::new(8.0, 0.0))))
            .add_child(Box::new(bar_chart))
        ))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 12.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(pie_chart))
            .add_child(Box::new(Spacer::new(Vec2::new(16.0, 0.0))))
            .add_child(Box::new(Column::new()
                .add_child(Box::new(Label::new("Trend")))
                .add_child(Box::new(sparkline))
            ))
        ));
    
    let charts_panel = Panel::new(Box::new(charts_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // DATA PANEL
    // =========================================================================
    
    let table = Table::new(vec![
        TableColumn::new("Name", 90.0),
        TableColumn::new("Status", 70.0),
        TableColumn::new("CPU", 50.0),
    ])
    .with_rows(vec![
        TableRow::new(vec!["Server A", "Online", "85%"]),
        TableRow::new(vec!["Server B", "Online", "72%"]),
        TableRow::new(vec!["Server C", "Offline", "0%"]),
        TableRow::new(vec!["Database", "Online", "91%"]),
    ]);
    
    let list = ListView::new()
        .with_items(vec![
            ListItem::new("Documents"),
            ListItem::new("Downloads"),
            ListItem::new("notes.txt"),
            ListItem::new("config.json"),
        ]);
    
    let data_content = Column::new()
        .add_child(Box::new(Label::new("Data Widgets")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(Row::new()
            .add_child(Box::new(table))
            .add_child(Box::new(Spacer::new(Vec2::new(8.0, 0.0))))
            .add_child(Box::new(list))
        ));
    
    let data_panel = Panel::new(Box::new(data_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // CONTROLS PANEL
    // =========================================================================
    
    let progress = ProgressBar::new(0.72).with_color(Vec4::new(0.3, 0.85, 0.5, 1.0));
    let toggle = Toggle::new("Enable", true);
    let number = NumberInput::new(50.0).with_range(0.0, 100.0).with_step(10.0);
    let radio = RadioGroup::new(vec![
        "Low".to_string(), "Medium".to_string(), "High".to_string()
    ]).with_selected(1);
    
    let controls_content = Column::new()
        .add_child(Box::new(Label::new("Premium Controls")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(progress))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(toggle))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(number))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 6.0))))
        .add_child(Box::new(radio));
    
    let controls_panel = Panel::new(Box::new(controls_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // RICH TEXT PANEL
    // =========================================================================
    
    let rich_display = RichText::from_markdown(
        "Welcome to **GlassUI**! Supports *italic* and `code`."
    );
    
    let rich_editor = RichTextEditor::new()
        .with_size(300.0, 100.0)
        .with_content("Type here...");
    
    let richtext_content = Column::new()
        .add_child(Box::new(Label::new("Rich Text")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(rich_display))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(rich_editor));
    
    let richtext_panel = Panel::new(Box::new(richtext_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // INPUT PANEL
    // =========================================================================
    
    let text_input = TextInput::new("Enter text...");
    let slider = Slider::new(0.65);
    let checkbox = Checkbox::new("Dark Mode", true);
    let dropdown = Dropdown::new(vec![
        "Option A".to_string(), "Option B".to_string(), "Option C".to_string()
    ]);
    
    let inputs_content = Column::new()
        .add_child(Box::new(Label::new("Inputs")))
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
    // TABS PANEL
    // =========================================================================
    
    let tab1 = Column::new()
        .add_child(Box::new(Label::new("General")))
        .add_child(Box::new(Slider::new(0.5)))
        .add_child(Box::new(Toggle::new("Auto", false)));
    
    let tab2 = Column::new()
        .add_child(Box::new(Label::new("Display")))
        .add_child(Box::new(Checkbox::new("HiDPI", true)));
    
    let tabs = TabBar::new()
        .add_tab("General", Box::new(tab1))
        .add_tab("Display", Box::new(tab2));
    
    let tabs_content = Column::new()
        .add_child(Box::new(Label::new("Tabs")))
        .add_child(Box::new(Spacer::new(Vec2::new(0.0, 8.0))))
        .add_child(Box::new(tabs));
    
    let tabs_panel = Panel::new(Box::new(tabs_content))
        .with_color(Vec4::new(0.06, 0.06, 0.1, 0.92));
    
    // =========================================================================
    // HEADER
    // =========================================================================
    
    let header = Panel::new(Box::new(Row::new()
        .add_child(Box::new(Label::new("GlassUI Framework Demo")))
        .add_child(Box::new(Spacer::new(Vec2::new(40.0, 0.0))))
        .add_child(Box::new(Label::new("Charts | Data | Controls | RichText | Tabs")))
    ))
    .with_color(Vec4::new(0.04, 0.04, 0.08, 0.95));
    
    // =========================================================================
    // ROOT LAYOUT - All panels draggable
    // =========================================================================
    
    let mut root = Stack::new()
        .add_child(Box::new(Align::new(Alignment::TopLeft, Box::new(header))))
        .add_child(Box::new(Align::new(Alignment::TopLeft,
            Box::new(Draggable::new(Box::new(Resizable::new(
                Box::new(charts_panel), Vec2::new(560.0, 340.0))))))))
        .add_child(Box::new(Align::new(Alignment::TopRight,
            Box::new(Draggable::new(Box::new(Resizable::new(
                Box::new(data_panel), Vec2::new(440.0, 220.0))))))))
        .add_child(Box::new(Align::new(Alignment::BottomLeft,
            Box::new(Draggable::new(Box::new(controls_panel))))))
        .add_child(Box::new(Align::new(Alignment::Center,
            Box::new(Draggable::new(Box::new(inputs_panel))))))
        .add_child(Box::new(Align::new(Alignment::BottomRight,
            Box::new(Draggable::new(Box::new(richtext_panel))))))
        .add_child(Box::new(Align::new(Alignment::TopRight,
            Box::new(Draggable::new(Box::new(tabs_panel))))));
    
    let mut cursor_pos = Vec2::ZERO;

    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);

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
                context.update(0.016);
                root.update(0.016);
                root.layout(Vec2::ZERO, Vec2::new(context.width as f32, context.height as f32));
                context.render(&mut root);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {
                root.handle_event(&event, cursor_pos);
            }
        }
    }).unwrap();
}
