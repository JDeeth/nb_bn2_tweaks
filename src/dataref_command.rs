use std::{cell::RefCell, rc::Rc};

use xplm::{
    command::{CommandHandler, OwnedCommand},
    debugln,
};

use crate::third_party_dataref::ThirdPartyDataref;

struct SetDataRefCommand {
    dataref: Rc<RefCell<ThirdPartyDataref>>,
    set_to: u32,
    reset_to: Option<u32>,
}

impl SetDataRefCommand {
    fn make_command(
        dataref: Rc<RefCell<ThirdPartyDataref>>,
        command_name: &str,
        command_description: &str,
        set_to: u32,
        reset_to: Option<u32>,
    ) -> OwnedCommand {
        debugln!("SetDataRefCmd {}", command_name);
        OwnedCommand::new(
            command_name,
            command_description,
            Self {
                dataref,
                set_to,
                reset_to,
            },
        )
        .unwrap()
    }
}

impl CommandHandler for SetDataRefCommand {
    fn command_begin(&mut self) {
        self.dataref.borrow_mut().set(self.set_to);
    }
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {
        if let Some(reset) = self.reset_to {
            self.dataref.borrow_mut().set(reset);
        }
    }
}

pub fn push_command(
    dataref_name: &str,
    command_name: &str,
    command_description: &str,
) -> OwnedCommand {
    SetDataRefCommand::make_command(
        Rc::new(RefCell::new(ThirdPartyDataref::new(
            dataref_name.to_owned(),
        ))),
        command_name,
        command_description,
        1,
        Some(0),
    )
}

struct ToggleAction {
    dataref: Rc<RefCell<ThirdPartyDataref>>,
}

impl ToggleAction {
    pub fn make_command(
        dataref: Rc<RefCell<ThirdPartyDataref>>,
        command_name: &str,
        command_description: &str,
    ) -> OwnedCommand {
        OwnedCommand::new(command_name, command_description, Self { dataref }).unwrap()
    }
}

impl CommandHandler for ToggleAction {
    fn command_begin(&mut self) {
        let mut dataref = self.dataref.borrow_mut();
        match dataref.get() {
            Some(0) => dataref.set(1),
            Some(_) => dataref.set(0),
            None => (),
        };
    }
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {}
}
pub struct ToggleSwitch {
    _dataref: Rc<RefCell<ThirdPartyDataref>>,
    _on: OwnedCommand,
    _off: OwnedCommand,
    _toggle: OwnedCommand,
}

impl ToggleSwitch {
    pub fn new(dataref_name: &str, command_prefix: &str, switch_name: &str) -> Self {
        let dataref = Rc::new(RefCell::new(ThirdPartyDataref::new(
            dataref_name.to_owned(),
        )));
        let on = SetDataRefCommand::make_command(
            Rc::clone(&dataref),
            &format!("{command_prefix}_on"),
            &format!("{switch_name}: set ON"),
            1,
            None,
        );
        let off = SetDataRefCommand::make_command(
            Rc::clone(&dataref),
            &format!("{command_prefix}_off"),
            &format!("{switch_name}: set OFF"),
            0,
            None,
        );
        let toggle = ToggleAction::make_command(
            Rc::clone(&dataref),
            &format!("{command_prefix}_toggle"),
            &format!("{switch_name}: toggle"),
        );
        Self {
            _dataref: dataref,
            _on: on,
            _off: off,
            _toggle: toggle,
        }
    }
}
