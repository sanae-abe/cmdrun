#![no_main]

use libfuzzer_sys::fuzz_target;
use cmdrun::command::interpolation::InterpolationContext;
use ahash::AHashMap;

fuzz_target!(|data: &[u8]| {
    // Convert fuzzing input to UTF-8 string (ignore invalid UTF-8)
    if let Ok(input) = std::str::from_utf8(data) {
        // Test 1: Basic interpolation (non-strict mode)
        let ctx = InterpolationContext::new(false);
        let _ = ctx.interpolate(input);

        // Test 2: Strict mode interpolation
        let strict_ctx = InterpolationContext::new(true);
        let _ = strict_ctx.interpolate(input);

        // Test 3: With environment variables
        let mut env = AHashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        env.insert("USER".to_string(), "fuzzer".to_string());
        env.insert("HOME".to_string(), "/home/fuzzer".to_string());
        env.insert("EMPTY".to_string(), String::new());
        env.insert("1".to_string(), "arg1".to_string());
        env.insert("2".to_string(), "arg2".to_string());

        let env_ctx = InterpolationContext::new(false).with_env_map(env.clone());
        let _ = env_ctx.interpolate(input);

        // Test 4: Strict mode with environment
        let strict_env_ctx = InterpolationContext::new(true).with_env_map(env);
        let _ = strict_env_ctx.interpolate(input);

        // Test 5: Test nested variable expansion (potential security issue)
        if input.len() < 1000 {  // Limit input size to prevent DoS
            let mut nested_env = AHashMap::new();
            nested_env.insert("VAR1".to_string(), input.to_string());
            nested_env.insert("VAR2".to_string(), format!("${{{}}}", input));

            let nested_ctx = InterpolationContext::new(false).with_env_map(nested_env);
            let _ = nested_ctx.interpolate(input);
        }
    }
});
