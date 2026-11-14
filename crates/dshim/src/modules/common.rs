//! Helpers shared by modules.

use crate::site::{Resolved, Resolver};
use dshim_profiles::{hex, Kind};
use std::io::Write;

/// Resolve and byte-check every site of a coupled-lever feature before anything is written. If
/// any site is missing or its bytes differ, log which and end the process: a requested feature is
/// applied in full or the run does not happen.
pub fn require_all(
    res: &Resolver,
    feature: &str,
    names: &[&str],
    log: &mut dyn Write,
) -> Vec<Resolved> {
    let mut out = Vec::with_capacity(names.len());
    for n in names {
        match res.site(n, log) {
            Some(r) => out.push(r),
            None => {
                lerr!(
                    log,
                    feature,
                    "site {n} unavailable, nothing applied, stopping"
                );
                let _ = log.flush();
                std::process::exit(3);
            }
        }
    }
    out
}

/// Write a Patch site's `new` bytes at its resolved, byte-checked address; a site the profile
/// lacks is skipped.
pub fn apply_patch(res: &Resolver, tool: &str, name: &str, log: &mut dyn Write) {
    let Some(r) = res.site(name, log) else {
        return;
    };
    if let Kind::Patch { new } = r.kind {
        if crate::write_code(r.addr, &hex(new)) {
            lok!(log, tool, "patch {name} @ {:#x} ({new})", r.addr);
        } else {
            lerr!(log, tool, "patch {name} write failed");
        }
    }
}
