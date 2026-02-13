//! Data model types returned by the Subsonic / OpenSubsonic REST API.
//!
//! Types are organised into sub-modules mirroring the API documentation sections.
//! All types derive [`serde::Deserialize`] and [`serde::Serialize`] for JSON round-tripping,
//! as well as [`Debug`], [`Clone`], and [`PartialEq`].

mod common;
mod browsing;
mod media;
mod playlists;
mod search;
mod sharing;
mod podcast;
mod jukebox;
mod radio;
mod chat;
mod user;
mod bookmarks;
mod scanning;
mod lyrics;
mod transcoding;

pub use common::*;
pub use browsing::*;
pub use media::*;
pub use playlists::*;
pub use search::*;
pub use sharing::*;
pub use podcast::*;
pub use jukebox::*;
pub use radio::*;
pub use chat::*;
pub use user::*;
pub use bookmarks::*;
pub use scanning::*;
pub use lyrics::*;
pub use transcoding::*;
