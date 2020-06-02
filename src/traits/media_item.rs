use objc_foundation::INSObject;
use crate::traits::{ITLibArtist, ITLibArtwork, ITLibAlbum};
use crate::media_item::{ITLibMediaItemMediaKind, ITLibMediaItemLyricsContentRating, ITLibMediaItemPlayStatus, ITLibMediaItemLocationType};
use chrono::{Date, Utc};
use url::Url;

pub trait ITLibMediaItemTrait: INSObject {
    type Artist: ITLibArtist;
    type Album: ITLibAlbum;
    type Artwork: ITLibArtwork;
    type VideoInfo: ITLibMediaItemVideoInfo;

    fn title(&self) -> String;
    fn sort_title(&self) -> Option<String>;
    fn artist(&self) -> &Self::Artist;
    fn composer(&self) -> String;
    fn sort_composer(&self) -> Option<String>;
    fn rating(&self) -> i8;
    fn is_rating_computed(&self) -> bool;
    fn start_time(&self) -> u64;
    fn stop_time(&self) -> u64;
    fn album(&self) -> &Self::Album;
    fn genre(&self) -> String;
    fn kind(&self) -> Option<String>;
    fn media_kind(&self) -> ITLibMediaItemMediaKind;
    fn file_size(&self) -> u64;
    fn total_time(&self) -> u64;
    fn track_number(&self) -> u64;
    fn category(&self) -> Option<String>;
    fn description(&self) -> Option<String>;
    fn lyrics_content_rating(&self) -> ITLibMediaItemLyricsContentRating;
    fn content_rating(&self) -> Option<String>;
    fn modified_date(&self) -> Date<Utc>;
    fn added_date(&self) -> Date<Utc>;
    fn bitrate(&self) -> u64;
    fn sample_rate(&self) -> u64;
    fn beats_per_minute(&self) -> u64;
    fn play_count(&self) -> u64;
    fn last_played_date(&self) -> Date<Utc>;
    fn play_status(&self) -> ITLibMediaItemPlayStatus;
    fn location(&self) -> Url;
    fn has_artwork_available(&self) -> bool;
    fn artwork(&self) -> Option<&Self::Artwork>;
    fn comments(&self) -> Option<String>;
    fn is_purchased(&self) -> bool;
    fn is_cloud(&self) -> bool;
    fn is_drm_protected(&self) -> bool;
    fn is_video(&self) -> bool;
    fn video_info(&self) -> Option<&Self::VideoInfo>;
    fn release_date(&self) -> Option<Date<Utc>>;
    fn year(&self) -> u64;
    fn skip_count(&self) -> u64;
    fn skip_date(&self) -> Option<Date<Utc>>;
    fn voice_over_language(&self) -> Option<String>;
    fn volume_adjustment(&self) -> i64;
    fn volume_normalization_energy(&self) -> u64;
    fn is_user_disabled(&self) -> bool;
    fn grouping(&self) -> Option<String>;
    fn location_type(&self) -> ITLibMediaItemLocationType;
}

pub trait ITLibMediaItemVideoInfo: INSObject {}

