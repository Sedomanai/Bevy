enum Tab {
    Viewport, // Bevy 화면이 들어갈 독
    Hierarchy,
    Inspector,
}

struct MyTabViewer {
    // Bevy 렌더링 텍스처의 egui id
    viewport_texture_id: egui::TextureId,
}

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = Tab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport => {
                // Dock Size
                let size = ui.available_size();

                // Show Bevy rendered texture to gui image panel
                ui.image(egui::load::SizedTexture::new(
                    self.viewport_texture_id,
                    size,
                ));
            }
            Tab::Hierarchy => {
                ui.label("Hierarchy Panel");
            }
            Tab::Inspector => {
                ui.label("Inspector Panel");
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport => "Viewport".into(),
            Tab::Hierarchy => "Hierarchy".into(),
            Tab::Inspector => "Inspector".into(),
        }
    }
}
