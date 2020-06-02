use objc_foundation::INSObject;

pub trait ITLibAlbum: INSObject {
    fn title(self) -> Option<String>;
    fn sort_title(self) -> Option<String>;
    fn is_compilation(self) -> bool;
    fn disc_count(self) -> u64;
    fn disc_number(self) -> u64;
    fn rating(self) -> i8;
    fn is_rating_computed(self) -> bool;
    fn is_gapless(self) -> bool;
    fn track_count(self) -> u64;
    fn album_artist(self) -> Option<String>;
    fn sort_album_artist(self) -> Option<String>;
    fn persistent_id(self) -> u64;
}
