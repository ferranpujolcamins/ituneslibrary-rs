use url::Url;
use objc::runtime::{Object, Sel};
use objc_id::ShareId;
use objc_foundation::{object_struct, NSArray, INSArray, NSSharedArray, NSString, INSString};
use objc::{msg_send, sel, sel_impl, class};

use crate::error::Error;
use crate::traits;
use crate::media_item::ITLibMediaItem;


object_struct!(ITLibrary);

impl traits::ITLibraryTrait for ITLibrary {
    type MediaItem = ITLibMediaItem;

    fn new_with_api_version(api_version: &str) -> Result<Self, Error> {
        unsafe {
            let version = NSString::from_str("1.0");
            let mut error: *mut Object = msg_send![class!(NSError), alloc];
            let mut error2: *mut *mut Object = &mut error;
            let error3: *mut *mut *mut Object = &mut error2;
            let library: Self = msg_send![
                class!(ITLibrary),
                libraryWithAPIVersion:version error:error3
            ];
            Ok(library)
        }
    }

    unsafe fn new_with_api_version_and_options(api_version: String, options: ITLibInitOptions) -> Result<Self, Error> {
        unimplemented!()
    }

    unsafe fn artwork_for_media_file(self, media_file: Url) -> Option<&'static <<Self as
    traits::ITLibraryTrait>::MediaItem as traits::ITLibMediaItemTrait>::Artwork> {
        unimplemented!()
    }

    unsafe fn reload_data(self) -> bool {
        unimplemented!()
    }

    unsafe fn unload_data(self) {
        unimplemented!()
    }

    unsafe fn application_version(self) -> String {
        unimplemented!()
    }

    unsafe fn api_major_version(self) -> u64 {
        unimplemented!()
    }

    unsafe fn api_minor_version(self) -> u64 {
        unimplemented!()
    }

    unsafe fn media_folder_location(self) -> Option<Url> {
        unimplemented!()
    }

    unsafe fn music_folder_location(self) -> Option<Url> {
        unimplemented!()
    }

    unsafe fn should_show_content_rating(self) -> i8 {
        unimplemented!()
    }

    fn all_media_items(&self) -> Vec<ShareId<Self::MediaItem>> {
        unsafe {
            // let media_items: NSSharedArray<ITLibMediaItem> = msg_send![self, allMediaItems];
            let media_items: *mut Object = msg_send![self, allMediaItems];
            // media_items.to_shared_vec()
            vec![]
        }
    }
}

pub enum ITLibInitOptions {
    LazyLoadData,
    None
}
