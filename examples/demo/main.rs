use egui_glfw::AppWindow;
use triangle::TriangleWidget;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

mod triangle;

fn main() {
    let mut app_window = AppWindow::new_default(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut test_str =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let triangle = triangle::Triangle::new();
    let triangle_widget: TriangleWidget = triangle.into();
    let mut quit = false;
    let mut amplitude = 10.0;

    while !app_window.get_window().should_close() {
        app_window.begin_frame();
        let egui_context = app_window.get_egui_context();

        egui::Window::new("Egui with GLFW").show(egui_context, |ui| {
            egui::TopBottomPanel::top("Top").show(egui_context, |ui| {
                ui.menu_button("File", |ui| {
                    {
                        let _ = ui.button("test 1");
                    }
                    ui.separator();
                    {
                        let _ = ui.button("test 2");
                    }
                });
            });

            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(" ");
            ui.add(egui::Slider::new(&mut amplitude, 0.0..=50.0).text("Amplitude"));
            ui.label(" ");
            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        egui::CentralPanel::default().show(egui_context, |ui| {
            let response = ui
                .add(triangle_widget.clone())
                .on_hover_ui_at_pointer(|ui| {
                    ui.label("on_hover_ui_at_pointer");
                })
                .on_hover_ui(|ui| {
                    ui.label("on_hover_ui");
                });
            response.context_menu(|ui| {
                ui.label("Context menu");
                let btn = ui.button("button");
                if btn.clicked() {
                    println!("Button clicked");
                }
            });
            if response.hovered() {
                print!("Hovered\n");
            }
            response
        });

        app_window.end_frame();

        if quit {
            break;
        }
    }
}
