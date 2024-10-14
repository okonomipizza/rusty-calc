// tests/input_tests.rs
use std::io::{self, Write};
use std::process::{Command, Stdio};
use rusty_calc::get_user_input;

#[test]
pub fn test_get_user_input() {
    // Mock inputs as if they were provided by the user
    let inputs = vec![
        "First line input\n",
        "Second line input\n",
        ".\n", // This indicates the end of input
    ];

    // Create a new process for the integration test
    let mut child = Command::new("cargo")
        .arg("run") // Adjust this based on how your project is structured
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    // Write the mock inputs to the child process's stdin
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        for input in inputs {
            stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
        }
    }

    // Capture the output
    let output = child.wait_with_output().expect("Failed to read stdout");

    // Check that the output matches expected results (you can adjust this as needed)
    let result = String::from_utf8_lossy(&output.stdout);
    assert!(result.contains("First line input"));
    assert!(result.contains("Second line input"));
}
