//! Modules targeting dee_ddpjoc_encoder (DD+ with JOC: spatial coder, backbone render, OAMD).

mod features;

use super::common::apply_patch;
use crate::site::Resolver;
use std::io::Write;

pub fn run_all(res: &Resolver, log: &mut dyn Write) {
    // On every run; only the Windows profile has the site.
    if !crate::levers().ddpjoc.zero_fill_original {
        apply_patch(res, "ddpjoc", "ddpjoc.perf.zero_fill", log);
    }
    // DDPJOC_RATE_GATE_OFF=1 removes the Atmos data-rate policy gate. On an Online-Media build
    // this does not unlock rates above 1024: the rate-config map rejects them first, and
    // streaming is a single substream capped at 1024 by frmsiz. It is kept for builds with a
    // Blu-ray mode, so the gate is one lever rather than a special case.
    if crate::levers().ddpjoc.rate_gate_off {
        ldebug!(
            log,
            "ddpjoc",
            "removing Atmos data-rate policy gate (no effect on an Online-only build)"
        );
        apply_patch(res, "ddpjoc", "ddpjoc.rate.online_max", log);
    }
    features::install(res, log);
    super::timing::install_ddpjoc(res, log);
}
