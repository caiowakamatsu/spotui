use fake::Fake;
use fake::faker::lorem::en::Words;
use fake::faker::name::en::Name;

pub struct Track {
    pub name: String,
    pub artist: String,
    pub length_seconds: u32,
}

pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppFocus {
    Playlist,
    Tracks,
}

pub struct App {
    pub playlists: Vec<Playlist>,
    pub selected_playlist: String,
    pub selected_song: String,
    pub focus: AppFocus,
}

impl App {
    pub fn new() -> App {
        let playlists = (0..5)
            .map(|_| {
                let name: String = Words(2..4).fake::<Vec<String>>().join(" ");
                let tracks = (0..10)
                    .map(|_| {
                        let track_name: String = Words(2..5).fake::<Vec<String>>().join(" ");
                        let artist: String = Name().fake();
                        let length_seconds = (120..420).fake::<u32>();
                        Track {
                            name: track_name,
                            artist,
                            length_seconds,
                        }
                    })
                    .collect::<Vec<_>>();

                Playlist { name, tracks }
            })
            .collect::<Vec<_>>();

        let selected_playlist = playlists[0].name.clone();
        let selected_song = playlists[0].tracks[0].name.clone();
        App {
            playlists: playlists,
            selected_playlist: selected_playlist,
            selected_song: selected_song,
            focus: AppFocus::Playlist,
        }
    }

    pub fn focus_right(&mut self) -> () {
        self.focus = match self.focus {
            AppFocus::Playlist => AppFocus::Tracks,
            AppFocus::Tracks => AppFocus::Playlist,
        }
    }

    pub fn focus_left(&mut self) -> () {
        self.focus = match self.focus {
            AppFocus::Playlist => AppFocus::Tracks,
            AppFocus::Tracks => AppFocus::Playlist,
        }
    }
}
