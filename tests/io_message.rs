use iokit::{io_message_bridged_constant, io_message_bridged_constant_count, IoMessage};

#[test]
fn bridged_constants_match_the_rust_enum() {
    assert_eq!(
        usize::try_from(io_message_bridged_constant_count()).ok(),
        Some(IoMessage::all_known().len())
    );

    for (index, message) in (0_u32..).zip(IoMessage::all_known().iter().copied()) {
        assert_eq!(io_message_bridged_constant(index), message.as_raw());
        assert_eq!(IoMessage::from_raw(message.as_raw()), message);
    }
}
