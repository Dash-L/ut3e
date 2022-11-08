pub struct App {
    label: String,
    value: f32,
}

impl Default for App {
    fn default() -> Self {
	Self {
	    label: "Hello egui".to_string(),
	    value: 0.0,
	}
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
	Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	let Self { label, value } = self;

	egui::CentralPanel::default().show(ctx, |ui| {
	    ui.heading("Ut3e");
	    ui.text_edit_singleline(label);
	    ui.horizontal(|ui| {
		if ui.button("-").clicked() {
		    *value -= 1.0;
		}
		ui.label(format!("{value}"));
		if ui.button("+").clicked() {
		    *value += 1.0;
		}
	    });
	});
    }
}
