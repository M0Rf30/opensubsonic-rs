//! API endpoint implementations.
//!
//! Each sub-module corresponds to a section of the Subsonic REST API and provides
//! methods on [`crate::Client`].

mod system;
mod browsing;
pub mod lists;
mod searching;
mod playlists;
mod media_retrieval;
mod media_annotation;
mod sharing;
mod podcast;
pub mod jukebox;
mod internet_radio;
mod chat;
mod user_management;
mod bookmarks;
mod scanning;
mod transcoding;
