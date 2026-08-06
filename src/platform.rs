pub mod browser {
    use std::process::Command;

    use anyhow::{ Result, bail };

    pub fn open(url: &str) -> Result<()> {
        let status = if cfg!(target_os = "macos") {
            Command::new("open").arg(url).status()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "start", "", url]).status()
        } else {
            Command::new("xdg-open").arg(url).status()
        }?;

        if status.success() { Ok(()) } else { bail!("exited with {status}") }
    }
}

pub mod clipboard {
    use anyhow::Result;

    pub fn copy(text: impl Into<String>) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;

        clipboard.set_text(text.into())?;

        Ok(())
    }
}
