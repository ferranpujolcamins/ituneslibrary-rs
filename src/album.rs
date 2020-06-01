pub trait ITLibAlbum {
    unsafe fn title(self) -> Option<String>;
    unsafe fn sort_title(self) -> Option<String>;
    unsafe fn is_compilation(self) -> bool;
    unsafe fn disc_count(self) -> u64;
    unsafe fn disc_number(self) -> u64;
    unsafe fn rating(self) -> i8;
    unsafe fn is_rating_computed(self) -> bool;
    unsafe fn is_gapless(self) -> bool;
    unsafe fn track_count(self) -> u64;
    unsafe fn album_artist(self) -> Option<String>;
    unsafe fn sort_album_artist(self) -> Option<String>;
    unsafe fn persistent_id(self) -> u64;
}
