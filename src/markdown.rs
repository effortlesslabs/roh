use termimad::MadSkin;

pub fn print_markdown(markdown_text: &str) {
    let skin = MadSkin::default();
    skin.print_text(markdown_text);
}
