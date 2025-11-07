#![no_main]

use libfuzzer_sys::fuzz_target;
use cmdrun::security::validation::{CommandValidator, escape_shell_arg};

fuzz_target!(|data: &[u8]| {
    // Convert fuzzing input to UTF-8 string (ignore invalid UTF-8)
    if let Ok(input) = std::str::from_utf8(data) {
        // Test 1: Strict mode validation (default)
        let strict_validator = CommandValidator::new();
        let _ = strict_validator.validate(input);

        // Test 2: Non-strict mode validation
        let non_strict_validator = CommandValidator::new()
            .with_strict_mode(false);
        let _ = non_strict_validator.validate(input);

        // Test 3: Variable expansion allowed
        let var_validator = CommandValidator::new()
            .allow_variable_expansion()
            .with_strict_mode(false);
        let _ = var_validator.validate(input);

        // Test 4: Pipe and redirect allowed
        let pipe_validator = CommandValidator::new()
            .allow_pipe()
            .allow_redirect()
            .with_strict_mode(false);
        let _ = pipe_validator.validate(input);

        // Test 5: Custom max length
        if input.len() < 10000 {  // Reasonable limit
            let length_validator = CommandValidator::new()
                .with_max_length(input.len() + 1);
            let _ = length_validator.validate(input);
        }

        // Test 6: Shell argument escaping (critical for security)
        let escaped = escape_shell_arg(input);

        // Verify escaping doesn't crash on re-validation
        let escape_validator = CommandValidator::new();
        let _ = escape_validator.validate(&escaped);

        // Test 7: Custom forbidden words
        if input.len() < 100 {
            let custom_validator = CommandValidator::new()
                .add_forbidden_word(input);
            let _ = custom_validator.validate(input);
        }

        // Test 8: All features enabled
        let full_validator = CommandValidator::new()
            .allow_variable_expansion()
            .allow_pipe()
            .allow_redirect()
            .with_strict_mode(false)
            .with_max_length(8192);
        let _ = full_validator.validate(input);
    }
});
