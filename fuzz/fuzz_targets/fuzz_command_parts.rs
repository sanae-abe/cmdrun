#![no_main]

use libfuzzer_sys::fuzz_target;
use ahash::AHashMap;

fuzz_target!(|data: &[u8]| {
    // Convert fuzzing input to UTF-8 string (ignore invalid UTF-8)
    if let Ok(input) = std::str::from_utf8(data) {
        // Limit input size to prevent resource exhaustion
        if input.len() > 4096 {
            return;
        }

        // Test 1: Shell word parsing (critical for command injection prevention)
        let _ = shell_words::split(input);

        // Test 2: Command definition parsing (includes CommandSpec)
        let _ = toml::from_str::<cmdrun::config::schema::Command>(input);

        // Test 3: Path expansion
        if let Ok(expanded) = shellexpand::full(input) {
            // Test if expanded path is safe
            let _ = std::path::PathBuf::from(expanded.as_ref());
        }

        // Test 4: Environment variable parsing
        let mut env = AHashMap::new();
        env.insert("TEST".to_string(), input.to_string());

        // Test interpolation with potentially malicious env values
        let ctx = cmdrun::command::interpolation::InterpolationContext::new(false)
            .with_env_map(env);
        let _ = ctx.interpolate(input);

        // Test 5: Regex pattern matching (for file watching)
        if input.len() < 256 {
            let _ = regex::Regex::new(input);
            let _ = globset::Glob::new(input);
        }

        // Test 6: Command validation with interpolation result
        if let Ok(interpolated) = cmdrun::command::interpolation::interpolate(
            input,
            &AHashMap::new()
        ) {
            let validator = cmdrun::security::validation::CommandValidator::new();
            let _ = validator.validate(&interpolated);
        }
    }
});
