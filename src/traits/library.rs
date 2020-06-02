use objc_foundation::INSObject;
use url::Url;

use crate::error::Error;
use crate::traits::media_item::ITLibMediaItemTrait;
use crate::library::ITLibInitOptions;
use objc_id::ShareId;

pub trait ITLibraryTrait: INSObject where Self: std::marker::Sized {
    type MediaItem: ITLibMediaItemTrait;

    fn new_with_api_version(api_version: &str) -> Result<Self, Error>;
    unsafe fn new_with_api_version_and_options(
        api_version: String,
        options: ITLibInitOptions
    ) -> Result<Self, Error>;
    unsafe fn artwork_for_media_file(
        self,
        media_file: Url
    ) -> Option<&'static <<Self as ITLibraryTrait>::MediaItem as ITLibMediaItemTrait>::Artwork>;
    unsafe fn reload_data(self) -> bool;
    unsafe fn unload_data(self);
    unsafe fn application_version(self) -> String;
    unsafe fn api_major_version(self) -> u64;
    unsafe fn api_minor_version(self) -> u64;
    unsafe fn media_folder_location(self) -> Option<Url>;
    unsafe fn music_folder_location(self) -> Option<Url>;
    unsafe fn should_show_content_rating(self) -> i8;
    fn all_media_items(&self) -> Vec<ShareId<Self::MediaItem>>;
    // unsafe fn all_playlists(self) -> *mut *mut objc::runtime::Object;
}

// pub enum ITLibInitOptions {
//     LazyLoadData,
//     None
// }
