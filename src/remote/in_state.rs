use std::collections::VecDeque;
use std::sync::Mutex;
use cgmath::num_traits::abs;
use cgmath::{Deg, Rad};
use log::{info, warn, Level};
use once_cell::sync::Lazy;
use shipyard::Unique;

use crate::algo::{analyze_stp, P_UP_REVERSE};
use crate::algo::cnc::{all_to_stp, cnc_to_poly, LRACLR};
use crate::device::graphics::{Graphics, States};
use crate::device::graphics::States::{ChangeDornDir, FullAnimate, LoadLRA, ReadyToLoad, ReverseLRACLR, Dismiss, NewBendParams, SelectFromWeb};


static COMMANDS: Lazy<Mutex<CommandState>> = Lazy::new(|| Mutex::new(CommandState::new()));

struct CommandState {
    pub values: VecDeque<RemoteCommand>,
}

impl CommandState {
    pub fn new() -> Self {
        Self {
            values: VecDeque::new(),
        }
    }
    pub fn get_first(&mut self) -> Option<RemoteCommand> {
        self.values.remove(0)
    }
}
#[derive(Debug, Clone, PartialEq, )]
pub enum RemoteCommand {
    OnLoadSTPfile((Vec<u8>)),
    OnLoadLRAcommands((Vec<f32>)),
    OnSelectById(i32),
    OnInitBend((Vec<u8>)),
    OnDoBend,
    Reverse,
    ReverseDorn,
    OnChangeBendParams((Vec<f32>)),
    OnSelectByTable(i32),
    OnStpFileRequest((Vec<f32>)),
}



#[derive(Unique)]
pub struct InCmd {
    lraclr_arr: Vec<LRACLR>,
}
impl InCmd {
    pub fn new() -> InCmd {
        Self {
            lraclr_arr: vec![]
        }
    }
    pub fn check_curr_command(&mut self)->States {
        match COMMANDS.try_lock() {
            Ok(mut s) => {
                match s.get_first() {
                    None => { Dismiss }
                    Some(command) => {
                        match command {
                            RemoteCommand::OnLoadSTPfile(stp) => {
                                let  lracmds= analyze_stp(&stp);
                                if(lracmds.is_empty()){
                                    Dismiss
                                }else{
                                    ReadyToLoad( (lracmds,true))
                                }
                            }
                            RemoteCommand::OnSelectById(id) => {
                                Dismiss
                            }
                            RemoteCommand::OnInitBend (stp)=> {
                                Dismiss
                            }
                            RemoteCommand::OnDoBend => {
                                FullAnimate
                            }
                            RemoteCommand::Reverse => {
                                ReverseLRACLR
                            }
                            RemoteCommand::ReverseDorn => {
                                ChangeDornDir
                            }
                            RemoteCommand::OnLoadLRAcommands(lra) => {
                                LoadLRA(lra)
                            }
                            RemoteCommand::OnChangeBendParams(params) => {
                                NewBendParams(params)
                            }
                            RemoteCommand::OnSelectByTable(id) => {
                                SelectFromWeb(id)
                            }
                            RemoteCommand::OnStpFileRequest(v) => {
                                let mut lra_cmds: Vec<LRACLR> = vec![];
                                if (v.len() % 8 == 0 && !v.is_empty()) {
                                    v.chunks(8).for_each(|cmd| {
                                        let id1 = cmd[0];
                                        let id2 = cmd[1];
                                        let l = cmd[2];
                                        let lt = cmd[3];
                                        let r = cmd[4];
                                        let a = cmd[5];
                                        let clr = cmd[6];
                                        let pipe_radius = cmd[7];
                                        let lra_cmd = LRACLR {
                                            id1: id1.round() as i32,
                                            id2: id2.round() as i32,
                                            l: abs(l as f64),
                                            lt: Rad::from(Deg(a as f64)).0*clr as f64,
                                            r: r as f64,
                                            a: abs(a as f64),
                                            clr: abs(clr as f64),
                                            pipe_radius: abs(pipe_radius as f64),
                                        };
                                        lra_cmds.push(lra_cmd);
                                    });
                                }
                                let (cyls, tors) = cnc_to_poly(&lra_cmds, &P_UP_REVERSE);
                                let file=all_to_stp(&cyls,&tors);
                                Dismiss 
                            }
                        }
                    }
                }
            }
            Err(_) => { 
                warn!("CANT_LOCK") ;
                Dismiss
            }
        }
    }
}
unsafe impl Send for InCmd {}
unsafe impl Sync for InCmd {}
