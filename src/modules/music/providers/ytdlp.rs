use serde::{Deserialize, Serialize};
use songbird::input::{error::Result as SongbirdResult, Restartable};
use std::process::Command;

const YOUTUBE_DL_COMMAND: &str = "yt-dlp";

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItem {
    pub url: String,
    pub title: String,
    pub duration: i64,
    pub uploader: String,
}

pub struct YtDlp;

impl YtDlp {
    /// Search YouTube for the given query and return a list of matching URLs
    pub fn search(self, query: String) -> Vec<String> {
        // TODO: Figure out command
        let command = Command::new(YOUTUBE_DL_COMMAND)
            .args(["--flat-playlist", "--dump-json", &query])
            .output()
            .unwrap_or_else(|_| panic!("{} should be installed on the system", YOUTUBE_DL_COMMAND));

        vec![]
    }

    // TODO: Should this panic or shall we propagate the error?
    /// Get all URLs contained in the playlist residing behind the given URL
    pub fn playlist(url: String) -> Vec<PlaylistItem> {
        let command = Command::new(YOUTUBE_DL_COMMAND)
            .args(["--flat-playlist", "--dump-json", &url])
            .output()
            .unwrap_or_else(|_| panic!("{} should be installed on the system", YOUTUBE_DL_COMMAND));

        // TODO: Check if yt-dlp returned anything, else return None

        // NOTE: We need to split the output stream here because yt-dlp doesn't return a JSON
        // array, instead it returns a newline-separated list of JSON objects
        let videos: Vec<PlaylistItem> = command
            .stdout
            .split(|c| *c == b'\n')
            .filter_map(|entry| {
                if entry.is_empty() {
                    None
                } else {
                    serde_json::from_slice::<PlaylistItem>(entry).ok()
                }
            })
            .collect();

        videos
    }

    /// Load an audio stream from the given URL
    /// TODO: Return unified result (modified PlaylistItem) from here, used for optimizing
    /// CPU usage on long playlists and to properly split accountability of different parts of the code
    pub async fn url(url: String) -> SongbirdResult<Restartable> {
        // Use lazy restartable sources to not pay
        // for decoding of tracks which aren't actually live yet
        Restartable::ytdl(url, true).await
    }
}
