// crates/plato/src/textview.rs
use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, io, path::{Path, PathBuf}};

static CHECKBOX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\s*[-*]\s+\[)( |x)(\])").unwrap());

fn save_atomic(path: &Path, txt: &str) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, txt)?;
    fs::rename(tmp, path)?;
    Ok(())
}

fn toggle_checkbox(line: &mut String) -> bool {
    if let Some(cap) = CHECKBOX_RE.captures(line) {
        *line = if &cap[2] == " " {
            CHECKBOX_RE.replace(line, "${1}x${3}").into_owned()
        } else {
            CHECKBOX_RE.replace(line, "${1} ${3}").into_owned()
        };
        true
    } else {
        false
    }
}

pub struct TextView {
    path: PathBuf,
    lines: Vec<String>,
}

impl TextView {
    pub fn open(path: &Path) -> io::Result<Self> {
        let txt = fs::read_to_string(path)?;
        let mut lines: Vec<String> = txt.lines().map(|s| s.to_owned()).collect();
        if txt.ends_with('\n') { lines.push(String::new()); }
        Ok(Self { path: path.to_path_buf(), lines })
    }

    /// Call this from your tap handler after mapping y→line index.
    pub fn toggle_at_line(&mut self, line_idx: usize) -> io::Result<bool> {
        if let Some(line) = self.lines.get_mut(line_idx) {
            if toggle_checkbox(line) {
                let contents = self.lines.join("\n");
                save_atomic(&self.path, &contents)?;
                // TODO: trigger redraw in your scene system
                return Ok(true);
            }
        }
        Ok(false)
    }

    // TODO: integrate with your UI toolkit
    pub fn draw(&self) {
        // For first test, just no-op or log.
        // Later, render with the same text drawing utilities used elsewhere.
        // e.g., log::info!("Drawing {} lines", self.lines.len());
    }
}
