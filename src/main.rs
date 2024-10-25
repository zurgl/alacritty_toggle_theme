use std::io::{Read, Write};

const ALACRITTY_CONFIG_PATH: &str = "/home/zu/.alacritty.toml";
const ALACRITTY_THEME_PATH: &str = "~/.config/alacritty/themes/themes/";

const ALACRITTY_THEME_LIGHT: &str = "\t\"~/.config/alacritty/themes/themes/ayu_light.toml\"";
const ALACRITTY_THEME_DARK: &str = "\t\"~/.config/alacritty/themes/themes/ayu_dark.toml\"";

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open(ALACRITTY_CONFIG_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    std::fs::remove_file(ALACRITTY_CONFIG_PATH)?;
    let mut outfile = std::fs::File::create(ALACRITTY_CONFIG_PATH)?;
    contents.lines().for_each(|line| {
        if line.contains(ALACRITTY_THEME_PATH) {
            if line.contains(ALACRITTY_THEME_LIGHT) {
                outfile.write(ALACRITTY_THEME_DARK.as_bytes()).unwrap();
            } else {
                outfile.write(ALACRITTY_THEME_LIGHT.as_bytes()).unwrap();
            }
        } else {
            outfile.write(line.as_bytes()).unwrap();
        }
        outfile.write(b"\n").unwrap();
    });
    Ok(())
}
