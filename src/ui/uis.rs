use egui_extras::Column;
use shipyard::UniqueViewMut;
use crate::algo::{analyze_stp, cnc, P_UP_REVERSE};
use crate::algo::cnc::LRACLR;
use crate::device::graphics::{GlobalScene, GlobalState, UIOverlay};
use crate::device::graphics::States::ReadyToLoad;

pub fn wind1(ui_overlay: &UIOverlay){
    egui::Window::new("winit1 + egui + wgpu says hello!")
        .current_pos([100.0, 100.0])
        .resizable(true)
        .vscroll(true)
        .default_open(false)
        .show(ui_overlay.egui_renderer.context(), |ui| {
            ui.label("Label!");

            if ui.button("Button!").clicked() {
                println!("boom!")
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Pixels per point: {}",
                    ui_overlay.egui_renderer.context().pixels_per_point()
                ));
                if ui.button("-").clicked() {
                    //g_scene.scale_factor = (g_scene.scale_factor - 0.1).max(0.3);
                }
                if ui.button("+").clicked() {
                    //g_scene.scale_factor = (g_scene.scale_factor + 0.1).min(3.0);
                }
            });
        });
}

pub fn top_panel(ui_overlay: &UIOverlay, g_scene: &mut GlobalScene, gs: &mut GlobalState){
    egui::TopBottomPanel::top("my_panel").show(ui_overlay.egui_renderer.context(), |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("A").clicked(){
                g_scene.bend_step = 1;
                let stp: Vec<u8> = Vec::from((include_bytes!("../files/2.stp")).as_slice());
                let lraclr_arr = analyze_stp(&stp);
                let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                gs.state = ReadyToLoad((lraclr_arr, true));
                gs.v_up_orign = P_UP_REVERSE;
            };
            ui.separator();
            if ui.button("B").clicked(){
                println!("Hello B!");
            };
            ui.separator();
            if ui.button("C").clicked(){
                println!("Hello C!");
            };
            ui.separator();

        });
    });


/*    egui::TopBottomPanel::top("wrap_app_top_bar")
        //.frame(egui::Frame::default().inner_margin(4.0))
        .show(g_scene.egui_renderer.context(), |ui| {
            ui.horizontal_wrapped(|ui| {
                //ui.visuals_mut().button_frame = false;
                ui.separator();
                ui.menu_button("💻 Backend", |ui| {
                    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                });

                ui.separator();
            });
        });*/
}

pub fn left_panel(ui_overlay: &UIOverlay){
    egui::SidePanel::left("side_panel_left").show(ui_overlay.egui_renderer.context(), |ui| {
       /* egui_extras::TableBuilder::new(ui)
            .vscroll(false)
            .column(Column::auto())
            .column(Column::auto())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("column 1");
                });
                header.col(|ui| {
                    ui.strong("column 2");
                });
            })
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("See stack below A");
                    });
                    row.col(|ui| {
                        ui.label("See stack below B");
                    });
                });
            });*/
    });
}