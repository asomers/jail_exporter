// Module that quickly checks if rctl is available.
// This should be temporary while the upstream rctl module doesn't have
// FreeBSD 13 support.
use sysctl::{
    Ctl,
    CtlValue,
    Sysctl,
};

#[derive(Debug)]
pub enum RctlState {
    Disabled,
    Enabled,
    Jailed,
    NotPresent,
}

const CTL_KERN_RACCT_ENABLE: &str    = "kern.racct.enable";
const CTL_SECURITY_JAIL_JAILED: &str = "security.jail.jailed";

impl RctlState {
    pub fn check() -> Self {
        // Quick check to see if we're in a jail
        if Self::jailed() {
            return Self::Jailed;
        }

        // Now check for RCTL being available
        let res = Ctl::new(CTL_KERN_RACCT_ENABLE);

        // If any error occurs, we assume RCTL is not present
        let ctl = match res {
            Ok(ctl) => ctl,
            Err(_)  => return Self::NotPresent,
        };

        match ctl.value() {
            Ok(value) => {
                match value {
                    // FreeBSD 13 returns a U8 as the kernel variable is bool
                    CtlValue::U8(1) => Self::Enabled,

                    // FreeBSD older than 13 returns a Uint as the kernel
                    // variable is an int
                    CtlValue::Uint(1) => Self::Enabled,

                    // Anything else, it's disabled
                    _ => Self::Disabled,
                }
            },

            // Anything else, it's disabled
            _ => Self::Disabled,
        }
    }

    fn jailed() -> bool {
        let res = Ctl::new(CTL_SECURITY_JAIL_JAILED);

        // If any error occurs, assume we're jailed
        let ctl = match res {
            Ok(ctl) => ctl,
            Err(_)  => return true,
        };

        match ctl.value() {
            Ok(value) => {
                match value {
                    CtlValue::Int(1) => true,
                    _                => false,
                }
            },
            Err(_) => true,
        }
    }
}
