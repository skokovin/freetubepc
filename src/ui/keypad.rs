use std::num::ParseFloatError;
use cgmath::num_traits::signum;
use eframe::egui::{self, pos2, vec2, Button, Ui, Vec2};
use egui::TextBuffer;
use log::warn;
use crate::algo::cnc::LRACLR;
use crate::ui::keypad::Cmd::Dismiss;

#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    Dismiss,
    LRA((LRACLR, i32)),
    StrightSpeedCmd(usize),
    RotateSpeedCmd(usize),
    AngleSpeedCmd(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Transition {
    #[default]
    None,
    CloseOnNextFrame,
    CloseImmediately,
}


#[derive(Clone, Debug)]
struct State {
    pub open: bool,
    closable: bool,
    close_on_next_frame: bool,
    start_pos: egui::Pos2,
    focus: Option<egui::Id>,
    events: Option<Vec<egui::Event>>,
    digits: String,
    sign: String,
    pub cmd: Cmd,
    pub is_done: bool,
}

impl State {
    fn new() -> Self {
        Self {
            open: false,
            closable: false,
            close_on_next_frame: false,
            start_pos: pos2(100.0, 100.0),
            focus: None,
            events: None,
            digits: String::from("|"),
            sign: String::from("+"),
            cmd: Dismiss,
            is_done:false,
        }
    }
    fn set_default(&mut self) {
        self.open= false;
        self.closable= false;
        self.close_on_next_frame= false;
        self.start_pos= pos2(100.0, 100.0);
        self.focus= None;
        self.events= None;
        self.digits= String::from("|");
        self.sign= String::from("+");
        self.cmd= Dismiss;
        self.is_done=false;
    }
    fn queue_char(&mut self, c: char) {
        let events = self.events.get_or_insert(vec![]);
        if let Some(key) = egui::Key::from_name(&c.to_string()) {
            events.push(egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            });
        }
        events.push(egui::Event::Text(c.to_string()));
    }

    fn queue_key(&mut self, key: egui::Key) {
        let events = self.events.get_or_insert(vec![]);
        events.push(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        });
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple keypad widget.
pub struct Keypad {
    id: egui::Id,

}

impl Keypad {
    pub fn new() -> Self {
        Self {
            id: egui::Id::new("keypad"),
        }
    }

/*    pub fn bump_events(&self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let events = ctx.memory_mut(|m| {
            m.data.get_temp_mut_or_default::<State>(self.id).events.take()
        });
        if let Some(mut events) = events {
            events.append(&mut raw_input.events);
            raw_input.events = events;
        }
    }*/

    fn buttons(ui: &mut Ui, state: &mut State) -> Transition {
        let mut trans = Transition::None;
        ui.vertical(|ui| {
            let window_margin = ui.spacing().window_margin;
            let size_1x1 = vec2(32.0, 26.0);
            let _size_1x2 = vec2(32.0, 52.0 + window_margin.top);
            let _size_2x1 = vec2(64.0 + window_margin.left, 26.0);

            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.left);


            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("OK")).clicked() {
                    state.is_done=true;
                    state.open=false;
                }
                ui.label(state.sign.as_str());
                ui.label(state.digits.as_str());
            });

            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("1")).clicked() {
                    state.digits = state.digits.replace("|", "1|");
                    state.queue_char('1');
                }
                if ui.add_sized(size_1x1, Button::new("2")).clicked() {
                    state.digits = state.digits.replace("|", "2|");
                    state.queue_char('2');
                }
                if ui.add_sized(size_1x1, Button::new("3")).clicked() {
                    state.digits = state.digits.replace("|", "3|");
                    state.queue_char('3');
                }
                if ui.add_sized(size_1x1, Button::new("⏮")).clicked() {
                    let mut orig_str = String::from("|");
                    let str_tmp = state.digits.replace("|", "");
                    orig_str.push_str(str_tmp.as_str());
                    state.digits = orig_str;
                    state.queue_key(egui::Key::Home);
                }
                if ui.add_sized(size_1x1, Button::new("🔙")).clicked() {
                    match state.digits.find("|") {
                        None => {}
                        Some(index) => {
                            if index > 0 {
                                state.digits.remove(index - 1);
                            }
                        }
                    }
                    state.queue_key(egui::Key::Backspace);
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("4")).clicked() {
                    state.digits = state.digits.replace("|", "4|");
                    state.queue_char('4');
                }
                if ui.add_sized(size_1x1, Button::new("5")).clicked() {
                    state.digits = state.digits.replace("|", "5|");
                    state.queue_char('5');
                }
                if ui.add_sized(size_1x1, Button::new("6")).clicked() {
                    state.digits = state.digits.replace("|", "6|");
                    state.queue_char('6');
                }
                if ui.add_sized(size_1x1, Button::new("⏭")).clicked() {
                    let mut str_tmp = state.digits.replace("|", "");
                    str_tmp.push_str("|");
                    state.digits = str_tmp;
                    state.queue_key(egui::Key::End);
                }
                if ui.add_sized(size_1x1, Button::new("C")).clicked() {
                    state.digits = String::from("|");
                    state.queue_char('C');
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("7")).clicked() {
                    state.digits = state.digits.replace("|", "7|");
                    state.queue_char('7');
                }
                if ui.add_sized(size_1x1, Button::new("8")).clicked() {
                    state.digits = state.digits.replace("|", "8|");
                    state.queue_char('8');
                }
                if ui.add_sized(size_1x1, Button::new("9")).clicked() {
                    state.digits = state.digits.replace("|", "9|");
                    state.queue_char('9');
                }
                if ui.add_sized(size_1x1, Button::new("⏶")).clicked() {
                    state.queue_key(egui::Key::ArrowUp);
                }
                if ui.add_sized(size_1x1, Button::new("+/-")).clicked() {
                    if (state.sign == "+") {
                        state.sign = state.sign.replace("+", "-");
                    } else {
                        state.sign = state.sign.replace("-", "+");
                    }

                    //trans = Transition::CloseImmediately;
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("0")).clicked() {
                    state.digits = state.digits.replace("|", "0|");
                    state.queue_char('0');
                }
                if ui.add_sized(size_1x1, Button::new(".")).clicked() {
                    state.digits = state.digits.replace(".", "");
                    state.digits = state.digits.replace("|", ".|");
                    state.queue_char('.');
                }
                if ui.add_sized(size_1x1, Button::new("⏴")).clicked() {
                    match state.digits.find("|") {
                        None => {}
                        Some(index) => {
                            let mut str_tmp = state.digits.replace("|", "");
                            if index > 0 {
                                str_tmp.insert(index - 1, '|');
                                state.digits = str_tmp;
                            }
                        }
                    }

                    state.queue_key(egui::Key::ArrowLeft);
                }
                if ui.add_sized(size_1x1, Button::new("⏷")).clicked() {
                    state.queue_key(egui::Key::ArrowDown);
                }
                if ui.add_sized(size_1x1, Button::new("⏵")).clicked() {
                    match state.digits.find("|") {
                        None => {}
                        Some(index) => {
                            let mut str_tmp = state.digits.replace("|", "");
                            let l = str_tmp.len();
                            if index >= 0 && index < l {
                                str_tmp.insert(index + 1, '|');
                                state.digits = str_tmp;
                            }
                        }
                    }
                    state.queue_key(egui::Key::ArrowRight);
                }
            });
        });

        trans
    }

    pub fn show(&self, ctx: &egui::Context) {
        let (focus, mut state) = ctx.memory(|m| {
            (
                m.focused(),
                match m.data.get_temp::<State>(self.id) {
                    None => {
                        warn!("NONE STATE");
                        State::default()
                    }
                    Some(mut s) => {
                        s
                    }
                },
            )
        });
        //state.open=false;
        //warn!("FOCUS {:?}", state.open);
        let mut is_first_show = false;
        /*        if ctx.wants_keyboard_input() && state.focus != focus {
                    let y = ctx.style().spacing.interact_size.y * 1.25;
                    state.open = true;
                    state.start_pos = ctx.input(|i| {
                        i.pointer
                            .hover_pos()
                            .map_or(pos2(100.0, 100.0), |p| p + vec2(0.0, y))
                    });
                    state.focus = focus;
                    is_first_show = true;
                }*/

        if state.close_on_next_frame {
            state.open = false;
            state.close_on_next_frame = false;
            state.focus = None;
        }

        let mut open = state.open;

        let win = egui::Window::new("⌨ Keypad");
        //let win = egui::Window::new(state.digits.as_str());
        let win = if is_first_show {
            win.current_pos(state.start_pos)
        } else {
            win.default_pos(state.start_pos)
        };
        let resp = win.movable(true).resizable(false).open(&mut open).show(ctx, |ui| Self::buttons(ui, &mut state));

        state.open = open;

        if let Some(resp) = resp {
            match resp.inner {
                Some(Transition::CloseOnNextFrame) => {
                    state.close_on_next_frame = true;
                }
                Some(Transition::CloseImmediately) => {
                    state.open = false;
                    state.focus = None;
                }
                _ => {}
            }
            if !state.closable && resp.response.hovered() {
                state.closable = true;
            }
            if state.closable && resp.response.clicked_elsewhere() {
                state.open = false;
                state.closable = false;
                state.focus = None;
            }
            if is_first_show {
                ctx.move_to_top(resp.response.layer_id);
            }
        }

        if let (true, Some(focus)) = (state.open, state.focus) {
            ctx.memory_mut(|m| {
                m.request_focus(focus);
            });
        }

        ctx.memory_mut(|m| m.data.insert_temp(self.id, state));
    }

    pub fn set_open_close(&mut self, ctx: &egui::Context, cmd: Cmd) {
        ctx.memory_mut(|mut m| {
            let sts = m.data.get_temp_mut_or_default::<State>(self.id);
            if (sts.open) {
                sts.open = false;
            } else {
                sts.open = true;
                match cmd {
                    Dismiss => {}
                    Cmd::LRA((lra, indx)) => {
                        sts.cmd=Cmd::LRA((lra.clone(), indx));
                        let value= match indx {
                            0 => { lra.l }
                            1 => {lra.r}
                            2 => {lra.a}
                            3 => {lra.clr}
                            _ => {panic!("Invalid index")}
                        };
                        if (value < 0.0) {
                            sts.sign = String::from("-");
                        } else {
                            sts.sign = String::from("+");
                        }
                        sts.digits = value.abs().to_string();
                        sts.digits.push_str("|");
                    }
                    Cmd::StrightSpeedCmd(value) => {
                        sts.cmd=Cmd::StrightSpeedCmd(value);
                        sts.digits = value.to_string();
                        sts.digits.push_str("|");
                    }
                    Cmd::RotateSpeedCmd(value) => {
                        sts.cmd=Cmd::RotateSpeedCmd(value);
                        sts.digits = value.to_string();
                        sts.digits.push_str("|");
                    }
                    Cmd::AngleSpeedCmd(value) => {
                        sts.cmd=Cmd::AngleSpeedCmd(value);
                        sts.digits = value.to_string();
                        sts.digits.push_str("|");
                    }
                }
            }
        });
    }

    pub fn check(&self, ctx: &egui::Context) ->Cmd{

        ctx.memory_mut(|mut m| {
         let state=   m.data.get_temp_mut_or_default::<State>(self.id);
            if(state.is_done){
                let mut val_s=state.sign.clone();
                val_s.push_str(state.digits.replace("|","").as_str());
                let retval=match val_s.parse::<f64>() {
                    Ok(f) => {
                        match &state.cmd {
                            Dismiss => {
                                Cmd ::Dismiss
                            }
                            Cmd::LRA((lra,indx)) => {
                                match indx {
                                    0 => {
                                        let new_lra = LRACLR { l: f.abs(), ..lra.clone() };
                                        Cmd::LRA((new_lra,indx.clone()))
                                    }
                                    1 => {
                                        let new_lra = LRACLR { r: f, ..lra.clone() };
                                        Cmd::LRA((new_lra,indx.clone()))
                                    }
                                    2 => {
                                        let new_lra = LRACLR { a: f.abs(), ..lra.clone() };
                                        Cmd::LRA((new_lra,indx.clone()))
                                    }
                                    3 => {
                                        let new_lra = LRACLR { clr: f.abs(), ..lra.clone() };
                                        Cmd::LRA((new_lra,indx.clone()))
                                    }

                                    _ => { Cmd ::Dismiss}
                                }
                            }
                            Cmd::StrightSpeedCmd(value) => {
                                Cmd::StrightSpeedCmd(f as usize)
                            }
                            Cmd::RotateSpeedCmd(value) => {
                                Cmd::RotateSpeedCmd(f as usize)
                            }
                            Cmd::AngleSpeedCmd(value) => {
                                Cmd::AngleSpeedCmd(f as usize)
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Parse error {:?}",val_s);
                        Cmd ::Dismiss
                    }
                };
               warn!("VAL {:?}", retval);
                state.set_default();
                retval
            }
            else{
                Dismiss
            }
        })
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}
