use std::cell::RefCell;
use crate::wyrand::WyRand;

thread_local! {
    static THREAD_RNG: RefCell<Option<WyRand>> = RefCell::new(None);
}

/// This uses seed_from_system to generate a thread-local hash state. 
/// This is faster than calling seed_from_system, which is a system call on x86.
pub fn from_local() -> u64 {
    THREAD_RNG.with_borrow_mut(|state| {
        state.get_or_insert_with(|| WyRand::with_seed(from_system())).next()
    })
}

/// Generate an rng seed with getrandom on x86 and 
/// web_time::SystemTime on wasm. 
#[cfg(not(target_arch = "wasm32"))]
pub fn from_system() -> u64 {
    match getrandom::u64() {
        Ok(v) => v,
        // If getrandom fails to generate entropy, fall back to system time nanoseconds.
        Err(e) => {
            eprintln!("(just-rng) getrandom entropy failed with err: '{e:?}'. Falling back to SystemTime");
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            ((nanos >> 64) ^ nanos) as u64
        }
    }        
}

/// Generate an rng seed with getrandom on x86 and 
/// web_time::SystemTime on wasm. 
#[cfg(target_arch = "wasm32")]
pub fn from_system() -> u64 {
    use web_time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    ((nanos >> 64) ^ nanos) as u64
}
