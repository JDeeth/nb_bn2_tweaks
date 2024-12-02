use xplm::{
    command::{CommandHandler, OwnedCommand},
    data::{borrowed::DataRef, DataReadWrite as _, ReadWrite},
    debugln,
};

struct SetDataRefCommand {
    dataref_name: String,
    dataref: Option<DataRef<u32, ReadWrite>>,
    set_to: u32,
    reset_to: Option<u32>,
}

impl SetDataRefCommand {
    fn set(&mut self, pos: u32) {
        if self.dataref.is_none() {
            if let Ok(dataref) = DataRef::find(&self.dataref_name) {
                self.dataref = dataref.writeable().ok()
            }
        }
        if let Some(dataref) = self.dataref.as_mut() {
            dataref.set(pos);
        }
    }

    fn make(
        dataref_name: &str,
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
                dataref_name: dataref_name.to_string(),
                dataref: None,
                set_to,
                reset_to,
            },
        )
        .unwrap()
    }
}

impl CommandHandler for SetDataRefCommand {
    fn command_begin(&mut self) {
        self.set(self.set_to);
    }
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {
        if let Some(reset) = self.reset_to {
            self.set(reset);
        }
    }
}

pub fn push_command(
    dataref_name: &str,
    command_name: &str,
    command_description: &str,
) -> OwnedCommand {
    SetDataRefCommand::make(dataref_name, command_name, command_description, 1, Some(0))
}

pub fn toggle_switch(
    dataref_name: &str,
    command_prefix: &str,
    switch_name: &str,
) -> (OwnedCommand, OwnedCommand) {
    (
        SetDataRefCommand::make(
            dataref_name,
            &format!("{}_on", command_prefix),
            &format!("Set {} ON", switch_name),
            1,
            None,
        ),
        SetDataRefCommand::make(
            dataref_name,
            &format!("{}_off", command_prefix),
            &format!("Set {} OFF", switch_name),
            0,
            None,
        ),
    )
}
