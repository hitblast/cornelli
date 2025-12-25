use std::sync::atomic::{AtomicBool, Ordering};

/*
 * Atomic variables which are only set by global flags per process execution.
 * You may modify them according to your own needs.
 *
 */

// --accept-all
static ACCEPT_ALL: AtomicBool = AtomicBool::new(false);

pub fn set_accept_all(value: bool) {
    ACCEPT_ALL.store(value, Ordering::SeqCst);
}

pub fn should_accept_all() -> bool {
    ACCEPT_ALL.load(Ordering::SeqCst)
}
