use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use egui::Sense;
use egui_extras::Column;
use log::warn;
use shipyard::UniqueViewMut;
use crate::algo::{analyze_stp, cnc, P_UP_REVERSE};
use crate::algo::cnc::LRACLR;
use crate::device::graphics::{AnimState, GlobalScene, GlobalState, States, UIOverlay};
use crate::device::graphics::States::{FullAnimate, ReadyToLoad, ReverseLRACLR, SelectFromWeb};

pub fn wind1(ui_overlay: &UIOverlay) {
    egui::Window::new("winit1 + egui + wgpu says hello!").current_pos([100.0, 100.0]).resizable(true).vscroll(true).default_open(false).show(ui_overlay.egui_renderer.context(), |ui| {
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

pub fn top_panel(ui_overlay: &UIOverlay, g_scene: &mut GlobalScene, gs: &mut GlobalState) {
    egui::TopBottomPanel::top("my_panel").show(ui_overlay.egui_renderer.context(), |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("File").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("STEP", &["stp", "step"]).pick_file() {
                    match File::open(path) {
                        Ok(f) => {
                            let mut reader = BufReader::new(f);
                            let mut line = String::new();
                            reader.read_to_string(&mut line);
                            let stp = line.as_bytes().to_vec();
                            g_scene.bend_step = 1;
                            let lraclr_arr = analyze_stp(&stp);
                            gs.state = ReadyToLoad((lraclr_arr, true));
                            gs.v_up_orign = P_UP_REVERSE;
                        }
                        Err(_) => {}
                    }
                    //self.picked_path = Some(path.display().to_string());
                }
            };
            ui.separator();
            if ui.button("Reverse").clicked() {
                if (!gs.lraclr_arr_reversed.is_empty()) {
                    g_scene.bend_step = 1;
                    gs.state = ReverseLRACLR
                }
            };
            ui.separator();
            if ui.button("Simulate").clicked() {
                gs.anim_state = AnimState::default();
                g_scene.bend_step = 1;
                gs.state = FullAnimate
            };
            ui.separator();
            ui.menu_button("Demos", |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                if ui.button("Demo1").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/1.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo2").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/2.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo3").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/3.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo4").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/4.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo5").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/5.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo6").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/6.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo7").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/7.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo8").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/8.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo9").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/9.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo10").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/10.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo11").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/11.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo12").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/12.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo13").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/13.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo14").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/14.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo15").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/15.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
                if ui.button("Demo16").clicked() {
                    g_scene.bend_step = 1;
                    let stp: Vec<u8> = Vec::from((include_bytes!("../files/16.stp")).as_slice());
                    let lraclr_arr = analyze_stp(&stp);
                    let lraclr_arr_reversed: Vec<LRACLR> = cnc::reverse_lraclr(&lraclr_arr);
                    gs.state = ReadyToLoad((lraclr_arr, true));
                    gs.v_up_orign = P_UP_REVERSE;
                    ui.close_menu();
                };
            });
        });
    });
}



pub fn left_panel(ui_overlay: &UIOverlay, g_scene: &mut GlobalScene, gs: &mut GlobalState) {
    //warn!("op_counter {:?}",gs.anim_state.op_counter);
    let col_width = 50.0;
    let col_heigth = 8.0;
    egui::SidePanel::left("side_panel_left").min_width(400.0).show(ui_overlay.egui_renderer.context(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let color= egui::Color32::from_rgb(255, 255, 255);
                ui.add_sized([col_width, col_heigth],
                             egui::Label::new(egui::RichText::new("L").color(color))
                );
                ui.separator();
                ui.add_sized([col_width, col_heigth],
                             egui::Label::new(egui::RichText::new("R").color(color))
                );
                ui.separator();
                ui.add_sized([col_width, col_heigth],
                             egui::Label::new(egui::RichText::new("A").color(color))
                );
                ui.separator();
                ui.add_sized([col_width, col_heigth],
                             egui::Label::new(egui::RichText::new("Bend L").color(color))
                );
                ui.separator();
                ui.add_sized([col_width, col_heigth],
                             egui::Label::new(egui::RichText::new("Bend R").color(color))

                );
                ui.separator();
            });
            ui.separator();
            let commands = {
                if (!gs.is_reversed) {
                    gs.lraclr_arr.clone()
                } else {
                    gs.lraclr_arr_reversed.clone()
                }
            };
            let mut count = 0;
            commands.iter().for_each(|lraclr| {
                ui.horizontal_wrapped(|ui| {
                    if(gs.anim_state.op_counter==count &&  matches!(gs.state,FullAnimate)){
                         ui.add_sized([col_width, col_heigth], egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.l)).color(egui::Color32::from_rgb(255, 0, 0))));
                    }else{

                       let color={
                            if lraclr.id1==g_scene.selected_id{
                                 egui::Color32::from_rgb(0, 0, 255)
                            }else{
                                 egui::Color32::from_rgb(255, 255, 255)
                            }
                       };

                       if ui.add_sized([col_width, col_heigth],
                                       egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.l)).color(color))
                                         .sense(Sense::click())).clicked(){
                           gs.state = SelectFromWeb(lraclr.id1);
                       };
                    }

                    ui.separator();
                    if(gs.anim_state.op_counter==count+1  &&  matches!(gs.state,FullAnimate)){
                        ui.add_sized([col_width, col_heigth], egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.r)).color(egui::Color32::from_rgb(255, 0, 0))));
                    }else{

                        let color={
                            if lraclr.id1==g_scene.selected_id{
                                egui::Color32::from_rgb(0, 0, 255)
                            }else{
                                egui::Color32::from_rgb(255, 255, 255)
                            }
                        };

                        if ui.add_sized([col_width, col_heigth],
                                        egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.r)).color(color))
                                            .sense(Sense::click())).clicked(){
                            gs.state = SelectFromWeb(lraclr.id1);
                        };

                    }
                    ui.separator();
                    if(gs.anim_state.op_counter==count+2  &&  matches!(gs.state,FullAnimate)){
                        ui.add_sized([col_width, col_heigth], egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.a)).color(egui::Color32::from_rgb(255, 0, 0))));
                        ui.separator();
                        ui.add_sized([col_width, col_heigth], egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.lt)).color(egui::Color32::from_rgb(255, 0, 0))));
                        ui.separator();
                        ui.add_sized([col_width, col_heigth], egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.clr)).color(egui::Color32::from_rgb(255, 0, 0))));
                        ui.separator();
                    }else{

                        let color={
                            if lraclr.id2==g_scene.selected_id{
                                egui::Color32::from_rgb(0, 0, 255)
                            }else{
                                egui::Color32::from_rgb(255, 255, 255)
                            }
                        };

                        if ui.add_sized([col_width, col_heigth],
                                        egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.a)).color(color.clone()))
                                            .sense(Sense::click())).clicked(){
                            gs.state = SelectFromWeb(lraclr.id2);
                        };
                        ui.separator();
                        if ui.add_sized([col_width, col_heigth],
                                        egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.lt)).color(color.clone()))
                                            .sense(Sense::click())).clicked(){
                            gs.state = SelectFromWeb(lraclr.id2);
                        };
                        ui.separator();
                        if ui.add_sized([col_width, col_heigth],
                                        egui::Label::new(egui::RichText::new(format!("{:.1}", lraclr.clr)).color(color.clone()))
                                            .sense(Sense::click())).clicked(){
                            gs.state = SelectFromWeb(lraclr.id2);
                        };
                        ui.separator();

                    }

                });
                ui.separator();
                count = count + 3;
            });
        });
    });
}

