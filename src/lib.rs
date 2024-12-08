use anim_dataref::AnimDataref;
use dataref_command::{push_command, ToggleSwitch};
use update_loop::UpdateLoopHandler;
use xplm::{
    command::OwnedCommand,
    debugln,
    flight_loop::FlightLoop,
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};

mod anim_dataref;
mod dataref_command;
mod third_party_dataref;
mod update_loop;

extern crate xplm;

struct NimbusBN2Tweaks {
    _flap_up: OwnedCommand,
    _flap_down: OwnedCommand,
    _switches: [ToggleSwitch; 19],
    _update_loop: FlightLoop,
}

impl NimbusBN2Tweaks {
    fn new() -> NimbusBN2Tweaks {
        let _switch_anim = [
            AnimDataref::new(
                "nimbus/bn2/animation/value_aux_fuel_left".to_owned(),
                "nimbus/bn2/animation/anim_aux_fuel_left",
                10.0,
            ),
            AnimDataref::new(
                "nimbus/bn2/animation/value_aux_fuel_right".to_owned(),
                "nimbus/bn2/animation/anim_aux_fuel_right",
                10.0,
            ),
        ];
        let update_loop =
            UpdateLoopHandler::new(_switch_anim.into_iter().map(|x| Box::new(x) as _).collect());

        NimbusBN2Tweaks {
            _flap_up: push_command(
                "nimbus/bn2/wings/flaps_request_up",
                "jdeeth/nb_bn2_tweaks/airframe/flaps_switch_hold_up",
                "Flaps switch: push to UP",
            ),
            _flap_down: push_command(
                "nimbus/bn2/wings/flaps_request_dn",
                "jdeeth/nb_bn2_tweaks/airframe/flaps_switch_hold_down",
                "Flaps switch: push to DOWN",
            ),
            _switches: [
                // Engine
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_left_mag1",
                    "jdeeth/nb_bn2_tweaks/engine/mag_left_1",
                    "Magneto Left 1",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_left_mag2",
                    "jdeeth/nb_bn2_tweaks/engine/mag_left_2",
                    "Magneto Left 2",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_right_mag1",
                    "jdeeth/nb_bn2_tweaks/engine/mag_right_1",
                    "Magneto Right 1",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_right_mag2",
                    "jdeeth/nb_bn2_tweaks/engine/mag_right_2",
                    "Magneto Right 2",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_aux_fuel_left",
                    "jdeeth/nb_bn2_tweaks/engine/fuel_pump_left",
                    "Fuel Pump Left",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_aux_fuel_right",
                    "jdeeth/nb_bn2_tweaks/engine/fuel_pump_right",
                    "Fuel Pump Right",
                ),
                // Electrical
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_external_supply",
                    "jdeeth/nb_bn2_tweaks/electrics/external_supply",
                    "External Supply",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_right_gen",
                    "jdeeth/nb_bn2_tweaks/electrics/right_gen",
                    "Right Generator",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_left_gen",
                    "jdeeth/nb_bn2_tweaks/electrics/left_gen",
                    "Left Generator",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_battery",
                    "jdeeth/nb_bn2_tweaks/electrics/battery",
                    "Battery",
                ),
                // Lights
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_beacon_switch",
                    "jdeeth/nb_bn2_tweaks/lights/beacon",
                    "Beacon",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_landing_right",
                    "jdeeth/nb_bn2_tweaks/lights/landing_right",
                    "Right Landing Light",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_landing_left",
                    "jdeeth/nb_bn2_tweaks/lights/landing_left",
                    "Left Landing Light",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_pass_notices",
                    "jdeeth/nb_bn2_tweaks/lights/passenger_notices",
                    "Passenger Notices",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_cabin_lights",
                    "jdeeth/nb_bn2_tweaks/lights/cabin_lights",
                    "Cabin Lights",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_nav_lights",
                    "jdeeth/nb_bn2_tweaks/lights/nav_lights",
                    "Nav Lights",
                ),
                // Heat / Deice
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_prop_deice",
                    "jdeeth/nb_bn2_tweaks/airframe/prop_deice",
                    "Propellor De-icing",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_airframe_deice",
                    "jdeeth/nb_bn2_tweaks/airframe/airframe_deice",
                    "Airframe De-icing",
                ),
                ToggleSwitch::new(
                    "nimbus/bn2/animation/value_pitot_and_stall",
                    "jdeeth/nb_bn2_tweaks/airframe/pitot_stall_heater",
                    "Pitot and Stall Warning Heaters",
                ),
            ],
            _update_loop: update_loop.into_flightloop(),
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
        let version = env!("CARGO_PKG_VERSION");
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
            description: format!("{version} compiled {ts}Z, {debug} opt level {opt_level}"),
        }
    }
}

xplane_plugin!(NimbusBN2Tweaks);
