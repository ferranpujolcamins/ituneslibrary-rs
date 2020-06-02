use chrono::{Date, Utc};
use url::Url;
use objc::runtime::Object;
use objc_foundation::{object_struct};
use objc::{msg_send, sel, sel_impl};
use crate::traits;
use crate::album::ITLibAlbum;
use crate::artist::ITLibArtist;
use crate::artwork::ITLibArtwork;

object_struct!(ITLibMediaItem);

impl traits::ITLibMediaItemTrait for ITLibMediaItem {
    type Artist = ITLibArtist;
    type Album = ITLibAlbum;
    type Artwork = ITLibArtwork;
    type VideoInfo = ITLibMediaItemVideoInfo;

    fn title(&self) -> String {
        unimplemented!()
    }

    fn sort_title(&self) -> Option<String> {
        unimplemented!()
    }

    fn artist(&self) -> &Self::Artist {
        unsafe {
            msg_send![self, artist]
        }
    }

    fn composer(&self) -> String {
        unimplemented!()
    }

    fn sort_composer(&self) -> Option<String> {
        unimplemented!()
    }

    fn rating(&self) -> i8 {
        unimplemented!()
    }

    fn is_rating_computed(&self) -> bool {
        unimplemented!()
    }

    fn start_time(&self) -> u64 {
        unimplemented!()
    }

    fn stop_time(&self) -> u64 {
        unimplemented!()
    }

    fn album(&self) -> &Self::Album {
        unimplemented!()
    }

    fn genre(&self) -> String {
        unimplemented!()
    }

    fn kind(&self) -> Option<String> {
        unimplemented!()
    }

    fn media_kind(&self) -> ITLibMediaItemMediaKind {
        unimplemented!()
    }

    fn file_size(&self) -> u64 {
        unimplemented!()
    }

    fn total_time(&self) -> u64 {
        unimplemented!()
    }

    fn track_number(&self) -> u64 {
        unimplemented!()
    }

    fn category(&self) -> Option<String> {
        unimplemented!()
    }

    fn description(&self) -> Option<String> {
        unimplemented!()
    }

    fn lyrics_content_rating(&self) -> ITLibMediaItemLyricsContentRating {
        unimplemented!()
    }

    fn content_rating(&self) -> Option<String> {
        unimplemented!()
    }

    fn modified_date(&self) -> Date<Utc> {
        unimplemented!()
    }

    fn added_date(&self) -> Date<Utc> {
        unimplemented!()
    }

    fn bitrate(&self) -> u64 {
        unimplemented!()
    }

    fn sample_rate(&self) -> u64 {
        unimplemented!()
    }

    fn beats_per_minute(&self) -> u64 {
        unimplemented!()
    }

    fn play_count(&self) -> u64 {
        unimplemented!()
    }

    fn last_played_date(&self) -> Date<Utc> {
        unimplemented!()
    }

    fn play_status(&self) -> ITLibMediaItemPlayStatus {
        unimplemented!()
    }

    fn location(&self) -> Url {
        unimplemented!()
    }

    fn has_artwork_available(&self) -> bool {
        unimplemented!()
    }

    fn artwork(&self) -> Option<&Self::Artwork> {
        unimplemented!()
    }

    fn comments(&self) -> Option<String> {
        unimplemented!()
    }

    fn is_purchased(&self) -> bool {
        unimplemented!()
    }

    fn is_cloud(&self) -> bool {
        unimplemented!()
    }

    fn is_drm_protected(&self) -> bool {
        unimplemented!()
    }

    fn is_video(&self) -> bool {
        unimplemented!()
    }

    fn video_info(&self) -> Option<&Self::VideoInfo> {
        unimplemented!()
    }

    fn release_date(&self) -> Option<Date<Utc>> {
        unimplemented!()
    }

    fn year(&self) -> u64 {
        unimplemented!()
    }

    fn skip_count(&self) -> u64 {
        unimplemented!()
    }

    fn skip_date(&self) -> Option<Date<Utc>> {
        unimplemented!()
    }

    fn voice_over_language(&self) -> Option<String> {
        unimplemented!()
    }

    fn volume_adjustment(&self) -> i64 {
        unimplemented!()
    }

    fn volume_normalization_energy(&self) -> u64 {
        unimplemented!()
    }

    fn is_user_disabled(&self) -> bool {
        unimplemented!()
    }

    fn grouping(&self) -> Option<String> {
        unimplemented!()
    }

    fn location_type(&self) -> ITLibMediaItemLocationType {
        unimplemented!()
    }
}

object_struct!(ITLibMediaItemVideoInfo);
impl traits::ITLibMediaItemVideoInfo for ITLibMediaItemVideoInfo {}

pub enum ITLibMediaItemMediaKind {
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
    KindRingTone,
    KindSong,
    KindTVShow,
    KindUnknown,
    KindVoiceMemo,
    KindITunesU,
}

pub enum ITLibMediaItemLyricsContentRating {
    None,
    Explicit,
    Clean,
}

pub enum ITLibMediaItemPlayStatus {
    None,
    PartiallyPlayed,
    UnPlayed,
}

pub enum ITLibMediaItemLocationType {
    URL,
    File,
    Remote,
    Unknown,
}
