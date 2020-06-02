use objc_id::ShareId;
use objc::{msg_send, class, sel, sel_impl, Message};
use objc_foundation::{object_struct};
use crate::traits;

object_struct!(ITLibAlbum);

impl traits::ITLibAlbum for ITLibAlbum {
    fn title(self) -> Option<String> {
        unimplemented!()
    }

    fn sort_title(self) -> Option<String> {
        unimplemented!()
    }

    fn is_compilation(self) -> bool {
        unimplemented!()
    }

    fn disc_count(self) -> u64 {
        unimplemented!()
    }

    fn disc_number(self) -> u64 {
        unimplemented!()
    }

    fn rating(self) -> i8 {
        unimplemented!()
    }

    fn is_rating_computed(self) -> bool {
        unimplemented!()
    }

    fn is_gapless(self) -> bool {
        unimplemented!()
    }

    fn track_count(self) -> u64 {
        unimplemented!()
    }

    fn album_artist(self) -> Option<String> {
        unimplemented!()
    }

    fn sort_album_artist(self) -> Option<String> {
        unimplemented!()
    }

    fn persistent_id(self) -> u64 {
        unimplemented!()
    }
}
