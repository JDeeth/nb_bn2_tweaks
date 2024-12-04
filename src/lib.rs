use dataref_command::{push_command, ToggleSwitch};
use xplm::{
    command::OwnedCommand,
    debugln,
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};

mod dataref_command;

extern crate xplm;

struct NimbusBN2Tweaks {
    _flap_up: OwnedCommand,
    _flap_down: OwnedCommand,
    _mag_switches: [ToggleSwitch; 4],
    _fuel_pump_switches: [ToggleSwitch; 2],
}

impl NimbusBN2Tweaks {
    fn new() -> NimbusBN2Tweaks {
        NimbusBN2Tweaks {
            _flap_up: push_command(
                "nimbus/bn2/wings/flaps_request_up",
                "jdeeth/nb_bn2_tweaks/hold_flaps_switch_up",
                "Push flaps switch to Up position",
            ),
            _flap_down: push_command(
                "nimbus/bn2/wings/flaps_request_dn",
                "jdeeth/nb_bn2_tweaks/hold_flaps_switch_down",
                "Push flaps switch to Down position",
            ),
            _mag_switches: [
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_left_mag1",
                    "jdeeth/nb_bn2_tweaks/mag_left_1",
                    "Magneto Left 1",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_left_mag2",
                    "jdeeth/nb_bn2_tweaks/mag_left_2",
                    "Magneto Left 2",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_right_mag1",
                    "jdeeth/nb_bn2_tweaks/mag_right_1",
                    "Magneto Right 1",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_right_mag2",
                    "jdeeth/nb_bn2_tweaks/mag_right_2",
                    "Magneto Right 2",
                ),
            ],
            _fuel_pump_switches: [
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_aux_fuel_left",
                    "jdeeth/nb_bn2_tweaks/fuel_pump_left",
                    "Fuel Pump Left",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_aux_fuel_right",
                    "jdeeth/nb_bn2_tweaks/fuel_pump_right",
                    "Fuel Pump Right",
                ),
            ],
        }
    }
}

impl Plugin for NimbusBN2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("[NimbusBN2Tweaks] Plugin start starting...");

        Ok(NimbusBN2Tweaks::new())
    }

    fn info(&self) -> PluginInfo {
        let ts = env!("VERGEN_BUILD_TIMESTAMP")
            .get(0..16)
            .unwrap_or("1970-01-01T00:00");
        let debug = match env!("VERGEN_CARGO_DEBUG") {
            "true" => "debug",
            _ => "release",
        };
        let opt_level = env!("VERGEN_CARGO_OPT_LEVEL");

        PluginInfo {
            name: String::from("Nimbus BN2 Tweaks"),
            signature: String::from("com.jdeeth.nb-bn2-tweaks"),
            description: format!("Compiled {ts}Z, {debug} opt level {opt_level}"),
        }
    }
}

xplane_plugin!(NimbusBN2Tweaks);
