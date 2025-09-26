use std::process::Command;

pub struct Git {}

impl Git {
    pub fn fetch(path: &str) -> std::io::Result<()> {
        let output = Command::new("git")
            .arg("fetch")
            .current_dir(path)
            .output()?;
        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub fn add_all(path: &str) -> std::io::Result<()> {
        let output = Command::new("git")
            .args(&["add", "."])
            .current_dir(path)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub fn commit(path: &str, message: &str) -> std::io::Result<()> {
        let output = Command::new("git")
            .args(&["commit", "-m", message])
            .current_dir(path)
            .output()?;
        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub fn push(path: &str) -> std::io::Result<()> {
        let output = Command::new("git").arg("push").current_dir(path).output()?;
        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }
}
