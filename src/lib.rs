use xplm::{
    debugln,
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};

extern crate xplm;

struct NimbusBN2Tweaks;

impl Plugin for NimbusBN2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("[NimbusBN2Tweaks] Plugin start starting...");

        Ok(NimbusBN2Tweaks)
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
