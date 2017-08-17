use heapsize::HeapSizeOf;
use super::*;
use super::Key::Character;

known_heap_size!(0, Code);
known_heap_size!(0, CompositionState);
known_heap_size!(0, KeyState);
known_heap_size!(0, Location);
known_heap_size!(0, Modifiers);

impl HeapSizeOf for CompositionEvent {
    fn heap_size_of_children(&self) -> usize {
        self.data.heap_size_of_children()
    }
}

impl HeapSizeOf for KeyboardEvent {
    fn heap_size_of_children(&self) -> usize {
        self.key.heap_size_of_children()
    }
}

impl HeapSizeOf for Key {
    fn heap_size_of_children(&self) -> usize {
        if let Character(ref c) = *self {
            c.heap_size_of_children()
        } else {
            0
        }
    }
}

#[test]
fn test_keyboard_event_heap_size() {
    let ev = KeyboardEvent {
        state: KeyState::Down,
        key: Character("\u{03a9}".to_owned()),
        code: Code::Unidentified,
        location: Location::Standard,
        modifiers: Modifiers::empty(),
        repeat: false,
        is_composing: false,
    };
    assert_eq!(8, ev.heap_size_of_children());
}
