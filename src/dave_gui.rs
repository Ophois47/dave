use eframe::egui;

struct DaveApp {
	name: String,
	age: u32,
	show_confirmation_message: bool,
	allowed_to_close: bool,
}

impl Default for DaveApp {
	fn default() -> Self {
		Self {
			name: "David".to_owned(),
			age: 34,
			show_confirmation_message: false,
			allowed_to_close: false,
		}
	}
}

impl eframe::App for DaveApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Dave's Graphical Interface");
			ui.horizontal(|ui| {
				let name_label = ui.label("Your Name: ");
				ui.text_edit_singleline(&mut self.name)
					.labelled_by(name_label.id);
			});
			ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));
			if ui.button("Increment").clicked() {
				self.age += 1;
			}
			ui.label(
				format!(
					"Hello '{}'. Age: {}",
					self.name,
					self.age,
				),
			);
			ui.image(
				egui::include_image!(
					"../dave_conf/var/frog.png"
				),
			);
		});

		if ctx.input(|i| i.viewport().close_requested()) {
			if self.allowed_to_close {
				// Do Nothing. Close
			} else {
				ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
				self.show_confirmation_message = true;
			}
		}

		if self.show_confirmation_message {
			egui::Window::new("Really Quit?")
				.collapsible(false)
				.resizable(false)
				.show(ctx, |ui| {
					if ui.button("No").clicked() {
						self.show_confirmation_message = false;
						self.allowed_to_close = false;
					}

					if ui.button("Yes").clicked() {
						self.show_confirmation_message = false;
						self.allowed_to_close = true;
						ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
					}
				});
		};
	}
}

pub fn dave_gui() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		..Default::default()
	};

	eframe::run_native(
		"Dave's Rockin' Fun Time Application",
		options,
		Box::new(|cc| {
			// Give Image Support
			egui_extras::install_image_loaders(&cc.egui_ctx);
			Box::<DaveApp>::default()
		}),
	)
}
