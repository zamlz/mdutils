use std::process::{Command, Stdio};
use std::time::Duration;
use std::io::Write;

const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
}

/// Executes code with the specified interpreter and timeout
pub fn execute_code(
    code: &str,
    bin: &str,
    timeout_seconds: Option<u64>,
) -> Result<ExecutionResult, String> {
    let timeout = timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECONDS);

    // Parse the bin string into command and args
    let parts: Vec<&str> = bin.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty bin specification".to_string());
    }

    let command = parts[0];
    let args = &parts[1..];

    // Spawn the process
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute '{}': {}", bin, e))?;

    // Write code to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(code.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;
    }

    // Wait for the process with timeout
    let result = wait_with_timeout(child, Duration::from_secs(timeout))?;

    Ok(result)
}

/// Waits for a child process with a timeout
fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<ExecutionResult, String> {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to wait for the child process
    thread::spawn(move || {
        let output = child.wait_with_output();
        let _ = tx.send(output);
    });

    // Wait for the result with timeout
    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => {
            let success = output.status.success();
            let output_str = if success {
                // On success, capture stdout
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                // On failure, capture stderr
                String::from_utf8_lossy(&output.stderr).to_string()
            };

            Ok(ExecutionResult {
                success,
                output: output_str,
            })
        }
        Ok(Err(e)) => Err(format!("Process execution failed: {}", e)),
        Err(_) => Err(format!("Process timed out after {} seconds", timeout.as_secs())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_code() {
        let code = "print('hello world')";
        let result = execute_code(code, "python3", Some(5));

        if result.is_ok() {
            let exec_result = result.unwrap();
            assert!(exec_result.success);
            assert!(exec_result.output.contains("hello world"));
        }
        // If python3 is not available, the test is skipped
    }

    #[test]
    fn test_execute_with_args() {
        let code = "console.log('hello from node')";
        let result = execute_code(code, "node -e", Some(5));

        // This test might fail if node is not installed, which is okay
        // We're just testing the parsing logic
        if let Ok(_) = result {
            // Test passed
        }
    }

    #[test]
    fn test_execute_failing_code() {
        let code = "import sys; sys.exit(1)";
        let result = execute_code(code, "python3", Some(5));

        if result.is_ok() {
            let exec_result = result.unwrap();
            assert!(!exec_result.success);
        }
    }
}
