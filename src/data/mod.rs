//! Data model types returned by the Subsonic / OpenSubsonic REST API.
//!
//! Types are organised into sub-modules mirroring the API documentation sections.
//! All types derive [`serde::Deserialize`] and [`serde::Serialize`] for JSON round-tripping,
//! as well as [`Debug`], [`Clone`], and [`PartialEq`].

mod bookmarks;
mod browsing;
mod chat;
mod common;
mod jukebox;
mod lyrics;
mod media;
mod playlists;
mod podcast;
mod radio;
mod scanning;
mod search;
mod sharing;
mod transcoding;
mod user;

pub use bookmarks::*;
pub use browsing::*;
pub use chat::*;
pub use common::*;
pub use jukebox::*;
pub use lyrics::*;
pub use media::*;
pub use playlists::*;
pub use podcast::*;
pub use radio::*;
pub use scanning::*;
pub use search::*;
pub use sharing::*;
pub use transcoding::*;
pub use user::*;
