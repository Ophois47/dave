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
			ui.label(format!("Your Age:"));
			ui.add(egui::Slider::new(&mut self.age, 0..=2000).text("Years Old"));
			if ui.button("Increment").clicked() {
				self.age += 1;
			}
			ui.separator();
			ui.label(
				format!(
					"It's great to meet you, {}. You don't look {} years old.",
					self.name,
					self.age,
				),
			);
			ui.separator();
			if self.age == 2000 {
				ui.image(
					egui::include_image!(
						"../dave_conf/etc/daves_assets/textures/ttyom.png"
					),
				);
			} else {
				ui.image(
					egui::include_image!(
						"../dave_conf/etc/daves_assets/textures/frog.png"
					),
				);
			};
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
					ui.horizontal(|ui| {
						if ui.button("Yes").clicked() {
							self.show_confirmation_message = false;
							self.allowed_to_close = true;
							ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
						}
						if ui.button("No").clicked() {
							self.show_confirmation_message = false;
							self.allowed_to_close = false;
						}
					});
				});
		};
	}
}

pub fn dave_gui() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([400.0, 550.0])
			.with_min_inner_size([400.0, 250.0]),
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
