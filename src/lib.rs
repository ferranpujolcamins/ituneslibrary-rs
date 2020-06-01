extern crate ituneslibrary_sys;
extern crate objc;
extern crate objc_foundation;
extern crate chrono;

#[link(name = "iTunesLibrary", kind = "framework")]
extern {}

mod artist;
mod album;

use std::os::raw::c_void;
use objc::runtime::{Object, Sel, BOOL};
use objc_foundation::{INSArray, NSArray, NSString, INSString, NSObject};
use objc::{msg_send, class, sel, sel_impl, __send_message, Encode, Encoding};
use objc_id::{Owned, Id};
use ituneslibrary_sys::{id, ITLibrary, ITLibInitOptions};
use std::any::Any;
use std::ops::Deref;
use std::mem;
use chrono::{Date, Utc};
use url::Url;

use artist::ITLibArtist;
use album::ITLibAlbum;

fn t() {
    unsafe {
        let version = nsstring("1.0");
        let obj: *mut Object = msg_send![class!(ITLibrary), alloc];
        let mut error: *mut Object = msg_send![class!(NSError), alloc];
        let mut error2: *mut *mut Object = &mut error;
        let error3: *mut *mut *mut Object = &mut error2;
        let obc: *mut Object = obj.initWithAPIVersion_error_(
            version,
            error3
        );
        let v = obj.apiMajorVersion();
        println!("api version: {}", v);

        // let a = NSArray:: obj.allMediaItems() as &INSArray<Item = Any, Own = Owned>;
        let a: *mut NSArray<NSObject> = mem::transmute(obj.allMediaItems());
        let c = (*a).count();
        let z = &(*a)[2];
        println!("count: {}", (*a).count())
    }
}

unsafe fn from_obj_array<A>(obj_array: *mut Object) -> Id<A> where A: INSArray {
    let cls = A::class();
    let c: *mut Object = msg_send![obj_array, class];
    let obj: *mut A = msg_send![cls, arrayWithArray: obj_array];
    Id::from_retained_ptr(obj)
}

fn nsstring(string: &str) -> *mut Object {
    const UTF8_ENCODING: usize = 4;
    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, initWithBytes:bytes
                                                       length:string.len()
                                                     encoding:UTF8_ENCODING];
        return obj
    }
}

trait ITLibMediaItem {
    type Artist: ITLibArtist;
    type Album: ITLibAlbum;
    type Artwork: ITLibArtwork;
    type VideoInfo: ITLibMediaItemVideoInfo;

    unsafe fn title(&self) -> String;
    unsafe fn sort_title(&self) -> Option<String>;
    unsafe fn artist(&self) -> &Self::Artist;
    unsafe fn composer(&self) -> String;
    unsafe fn sort_composer(&self) -> Option<String>;
    unsafe fn rating(&self) -> i8;
    unsafe fn is_rating_computed(&self) -> bool;
    unsafe fn start_time(&self) -> u64;
    unsafe fn stop_time(&self) -> u64;
    unsafe fn album(&self) -> &Self::Album;
    unsafe fn genre(&self) -> String;
    unsafe fn kind(&self) -> Option<String>;
    unsafe fn media_kind(&self) -> ITLibMediaItemMediaKind;
    unsafe fn file_size(&self) -> u64;
    unsafe fn total_time(&self) -> u64;
    unsafe fn track_number(&self) -> u64;
    unsafe fn category(&self) -> Option<String>;
    unsafe fn description(&self) -> Option<String>;
    unsafe fn lyrics_content_rating(&self) -> ITLibMediaItemLyricsContentRating;
    unsafe fn content_rating(&self) -> Option<String>;
    unsafe fn modified_date(&self) -> Date<Utc>;
    unsafe fn added_date(&self) -> Date<Utc>;
    unsafe fn bitrate(&self) -> u64;
    unsafe fn sample_rate(&self) -> u64;
    unsafe fn beats_per_minute(&self) -> u64;
    unsafe fn play_count(&self) -> u64;
    unsafe fn last_played_date(&self) -> Date<Utc>;
    unsafe fn play_status(&self) -> ITLibMediaItemPlayStatus;
    unsafe fn location(&self) -> Url;
    unsafe fn has_artwork_available(&self) -> bool;
    unsafe fn artwork(&self) -> Option<&Self::Artwork>;
    unsafe fn comments(&self) -> Option<String>;
    unsafe fn is_purchased(&self) -> bool;
    unsafe fn is_cloud(&self) -> bool;
    unsafe fn is_drm_protected(&self) -> bool;
    unsafe fn is_video(&self) -> bool;
    unsafe fn video_info(&self) -> Option<&Self::VideoInfo>;
    unsafe fn release_date(&self) -> Option<Date<Utc>>;
    unsafe fn year(&self) -> u64;
    unsafe fn skip_count(&self) -> u64;
    unsafe fn skip_date(&self) -> Option<Date<Utc>>;
    unsafe fn voice_over_language(&self) -> Option<String>;
    unsafe fn volume_adjustment(&self) -> i64;
    unsafe fn volume_normalization_energy(&self) -> u64;
    unsafe fn is_user_disabled(&self) -> bool;
    unsafe fn grouping(&self) -> Option<String>;
    unsafe fn location_type(&self) -> ITLibMediaItemLocationType;
}

enum ITLibMediaItemMediaKind {
    KindAlertTone,
    KindAudioBook,
    KindBook,
    KindDigitalBooklet,
    KindHomeVideo,
    KindIOSApplication,
    KindInteractiveBooklet,
    KindMovie,
    KindMusicVideo,
    KindPDFBook,
    KindPDFBooklet,
    KindPodcast,
    KindRingtone,
    KindSong,
    KindTVShow,
    KindUnknown,
    KindVoiceMemo,
    KindITunesU,
}

enum ITLibMediaItemLyricsContentRating {
    None,
    Explicit,
    Clean,
}

enum ITLibMediaItemPlayStatus {
    None,
    PartiallyPlayed,
    Unplayed,
}

trait ITLibArtwork {}

trait ITLibMediaItemVideoInfo {}

enum ITLibMediaItemLocationType {
    URL,
    File,
    Remote,
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::{t};

    #[test]
    fn it_works() {
        t();
        println!("OK");
    }
}
