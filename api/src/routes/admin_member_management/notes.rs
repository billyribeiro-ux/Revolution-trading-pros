//! Both member route families use the same transactional note contract.
pub(super) use super::read_handlers::{
    add_note as create_note, notes as get_notes, remove_note as delete_note,
};
