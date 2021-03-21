mod editor;
mod terminal;
mod document;

use editor::Editor;

fn main() {
    Editor::default().run();
}
