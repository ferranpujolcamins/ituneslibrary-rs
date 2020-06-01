pub trait ITLibArtist {
    unsafe fn name(self) -> Option<String>;
    unsafe fn sort_name(self) -> *mut *mut objc::runtime::Object;
    unsafe fn persistent_id(self) -> u64;
}
