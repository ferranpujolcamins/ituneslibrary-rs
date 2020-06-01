extern crate ituneslibrary_sys;
extern crate objc;
extern crate objc_foundation;

use std::os::raw::c_void;
use objc::runtime::{Object, Sel, BOOL};
use objc_foundation::{INSArray, NSArray, NSString, INSString, NSObject};
use objc::{msg_send, class, sel, sel_impl, __send_message, Encode, Encoding};
use objc_id::{Owned, Id};
use ituneslibrary_sys::{id, ITLibrary, ITLibInitOptions};
use std::any::Any;
use std::ops::Deref;
use std::mem;

#[link(name = "iTunesLibrary", kind = "framework")]
extern {}

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

struct ITLibMediaItem {
    obj: *mut Object
}

impl ituneslibrary_sys::ITLibMediaItem for ITLibMediaItem {
    unsafe fn title(self) -> String {
        let a = self.obj.title();
    }
    unsafe fn sortTitle(self) -> String { todo!() }
    unsafe fn artist(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn composer(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn sortComposer(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn rating(self) -> i64 { todo!() }
    unsafe fn isRatingComputed(self) -> i8 { todo!() }
    unsafe fn startTime(self) -> u64 { todo!() }
    unsafe fn stopTime(self) -> u64 { todo!() }
    unsafe fn album(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn genre(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn kind(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn mediaKind(self) -> u64 { todo!() }
    unsafe fn fileSize(self) -> u64 { todo!() }
    unsafe fn size(self) -> u64 { todo!() }
    unsafe fn totalTime(self) -> u64 { todo!() }
    unsafe fn trackNumber(self) -> u64 { todo!() }
    unsafe fn category(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn description(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn lyricsContentRating(self) -> u64 { todo!() }
    unsafe fn contentRating(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn modifiedDate(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn addedDate(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn bitrate(self) -> u64 { todo!() }
    unsafe fn sampleRate(self) -> u64 { todo!() }
    unsafe fn beatsPerMinute(self) -> u64 { todo!() }
    unsafe fn playCount(self) -> u64 { todo!() }
    unsafe fn lastPlayedDate(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn playStatus(self) -> u64 { todo!() }
    unsafe fn location(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn hasArtworkAvailable(self) -> i8 { todo!() }
    unsafe fn artwork(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn comments(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn isPurchased(self) -> i8 { todo!() }
    unsafe fn isCloud(self) -> i8 { todo!() }
    unsafe fn isDRMProtected(self) -> i8 { todo!() }
    unsafe fn isVideo(self) -> i8 { todo!() }
    unsafe fn videoInfo(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn releaseDate(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn year(self) -> u64 { todo!() }
    unsafe fn fileType(self) -> u64 { todo!() }
    unsafe fn skipCount(self) -> u64 { todo!() }
    unsafe fn skipDate(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn voiceOverLanguage(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn volumeAdjustment(self) -> i64 { todo!() }
    unsafe fn volumeNormalizationEnergy(self) -> u64 { todo!() }
    unsafe fn isUserDisabled(self) -> i8 { todo!() }
    unsafe fn grouping(self) -> *mut *mut objc::runtime::Object { todo!() }
    unsafe fn locationType(self) -> u64 { todo!() }
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
