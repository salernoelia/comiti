use std::process::Command;

pub struct Git {}

impl Git {
    fn run_git_command(args: &[&str], path: &str, operation: &str) -> std::io::Result<()> {
        let output = Command::new("git").args(args).current_dir(path).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            let error_msg = if stderr.is_empty() && stdout.is_empty() {
                format!("Git {} failed with exit code: {}", operation, output.status)
            } else if !stderr.is_empty() {
                format!("Git {} failed: {}", operation, stderr.trim())
            } else {
                format!("Git {} failed: {}", operation, stdout.trim())
            };

            return Err(std::io::Error::new(std::io::ErrorKind::Other, error_msg));
        }
        Ok(())
    }

    pub fn fetch(path: &str) -> std::io::Result<()> {
        Self::run_git_command(&["fetch"], path, "fetch")
    }

    pub fn add_all(path: &str) -> std::io::Result<()> {
        Self::run_git_command(&["add", "."], path, "add")
    }

    pub fn commit(path: &str, message: &str) -> std::io::Result<()> {
        Self::run_git_command(&["commit", "-m", message], path, "commit")
    }

    pub fn push(path: &str) -> std::io::Result<()> {
        Self::run_git_command(&["push"], path, "push")
    }
}
