pub fn init_wasm_instant() {
    use bevy_platform::time::Instant;
    // SAFETY: you're promising this returns a monotonic-ish elapsed time.
    unsafe {
        Instant::set_elapsed(|| {
            // Try high-resolution monotonic time first
            let ms = web_sys::window()
                .and_then(|w| w.performance().ok())
                .map(|p| p.now())
                // Fallback (not strictly monotonic, but better than nothing)
                .unwrap_or_else(|| js_sys::Date::now());
            Duration::from_nanos((ms * 1_000_000.0) as u64)
        });
    }
}
