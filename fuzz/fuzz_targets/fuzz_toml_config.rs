#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert fuzzing input to UTF-8 string (ignore invalid UTF-8)
    if let Ok(input) = std::str::from_utf8(data) {
        // Test 1: Parse TOML config
        let _ = toml::from_str::<cmdrun::config::schema::CommandsConfig>(input);

        // Test 2: Parse as generic toml::Value first (catches TOML syntax issues)
        if let Ok(value) = toml::from_str::<toml::Value>(input) {
            // Try to deserialize the parsed value
            let _ = toml::Value::try_into::<cmdrun::config::schema::CommandsConfig>(value);
        }

        // Test 3: Test TOML editing (toml_edit crate)
        let _ = input.parse::<toml_edit::DocumentMut>();

        // Test 4: Partial config parsing (test individual sections)
        let _ = toml::from_str::<cmdrun::config::schema::GlobalConfig>(input);
        let _ = toml::from_str::<cmdrun::config::schema::Command>(input);
        let _ = toml::from_str::<cmdrun::config::schema::Hooks>(input);
    }
});
