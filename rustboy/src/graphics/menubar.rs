#[derive(Default)]
pub struct MenuBar {

}


impl MenuBar {
    pub fn new() -> MenuBar {
        MenuBar {}
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        // egui::menu::bar(ui, |menu| {
        //     ui.menu_button("File", |ui| {
        //         if ui.button("Select Rom").clicked() {}
        //         if ui.button("Quit").clicked() {}
        //         ui.close_menu();
        //     });
        //     ui.menu_button("View", |ui| {});
        //     ui.menu_button("Controls", |ui| {});
        // });
    }
}