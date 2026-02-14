//! API endpoint implementations.
//!
//! Each sub-module corresponds to a section of the Subsonic REST API and provides
//! methods on [`crate::Client`].

mod bookmarks;
mod browsing;
mod chat;
mod internet_radio;
pub mod jukebox;
pub mod lists;
mod media_annotation;
mod media_retrieval;
mod playlists;
mod podcast;
mod scanning;
mod searching;
mod sharing;
mod system;
mod transcoding;
mod user_management;
