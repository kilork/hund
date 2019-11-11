use crate::config::HundSettings;

pub(crate) struct HundApp {
    settings: HundSettings,
}

impl HundApp {
    pub(crate) fn new() -> Self {
        let settings = HundSettings::load();
        HundApp { settings }
    }
}
