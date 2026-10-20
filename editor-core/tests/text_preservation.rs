use floem_editor_core::{buffer::{rope_text::RopeText, Buffer}, editor::EditType, selection::Selection};

#[test]
fn loading_preserves_all_line_endings() {
    let text = "first\rsecond\r\nthird\nlast";
    assert_eq!(Buffer::new(text).text().to_string(), text);
}

#[test]
fn edits_preserve_line_endings_through_undo_and_redo() {
    let mut buffer = Buffer::new("original\r\n");
    let addition = "one\rtwo\nthree\r\n";
    assert!(!buffer.can_undo());
    assert!(!buffer.can_redo());
    buffer.edit(&[(Selection::caret(buffer.len()), addition)], EditType::Other);
    let expected = format!("original\r\n{addition}");
    assert_eq!(buffer.text().to_string(), expected);
    assert!(buffer.can_undo());
    assert!(!buffer.can_redo());
    assert!(buffer.do_undo().is_some());
    assert_eq!(buffer.text().to_string(), "original\r\n");
    assert!(!buffer.can_undo());
    assert!(buffer.can_redo());
    assert!(buffer.do_redo().is_some());
    assert_eq!(buffer.text().to_string(), expected);
    assert!(buffer.can_undo());
    assert!(!buffer.can_redo());
}
