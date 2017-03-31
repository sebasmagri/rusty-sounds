use dbus::{BusType, Connection};
use dbus::tokio::TokioConnection;
use dbus::stdintf::OrgFreedesktopDBusProperties;

use futures::Future;

use relm;


#[derive(Debug, Clone)]
pub struct PlaybackMetadata {
    pub artist: String,
    pub album: String,
    pub title: String
}

impl PlaybackMetadata {
    fn new() -> PlaybackMetadata {
        PlaybackMetadata {
            artist: "".to_string(),
            album: "".to_string(),
            title: "".to_string()
        }
    }
}

pub fn get_playback_metadata() -> PlaybackMetadata {
    let c = Connection::get_private(BusType::Session).unwrap();
    let p = c.with_path("org.mpris.MediaPlayer2.mopidy",
                        "/org/mpris/MediaPlayer2", 5000);

    let mut pm = PlaybackMetadata::new();

    let metadata = p.get("org.mpris.MediaPlayer2.Player",
                         "Metadata").unwrap();

    let mut iter = metadata.0.as_iter().unwrap();

    while let Some(key) = iter.next() {
        let value = iter.next().unwrap();

        match key.as_str().unwrap() {
            "xesam:artist" => {
                let artists: Vec<String> = value.as_iter().unwrap()
                    .map(|variant| {
                        let mut inner_artists = Vec::<String>::new();
                        for inner_variant in variant.as_iter().unwrap() {
                            for artist in inner_variant.as_iter().unwrap() {
                                inner_artists.push(artist.as_str().unwrap().to_string());
                            }
                        }
                        inner_artists.join(" - ")
                    })
                    .collect();
                pm.artist = artists.join(" - ")
            },
            "xesam:album" => {
                pm.album = value.as_str().unwrap().to_string()
            },
            "xesam:title" => {
                pm.title = value.as_str().unwrap().to_string()
            },
            _ => ()
        }
    }

    pm
}

pub fn get_playback_status() -> String {
    let c = Connection::get_private(BusType::Session).unwrap();
    let p = c.with_path("org.mpris.MediaPlayer2.mopidy",
                        "/org/mpris/MediaPlayer2", 5000);

    let status = p.get("org.mpris.MediaPlayer2.Player",
                       "PlaybackStatus");

    match status {
        Ok(v) => {
            (v.0.as_str()).unwrap_or("Error").to_string()
        },
        Err(_) => {
            "Error".to_string()
        }
    }
}

// fn get_playback_metadata_future<'a>() -> impl Future<Item=Variant<Iter>, Error=()> + 'a {
    
// }


pub fn update_playback_future<'a>(method: &str, handle: &relm::Handle) -> impl Future<Item=(), Error=()> + 'a {
    let dbus = Connection::get_private(BusType::Session).unwrap();
    let dbus = TokioConnection::new(dbus, handle.clone());

    dbus
        .call("org.mpris.MediaPlayer2.mopidy",
              "/org/mpris/MediaPlayer2",
              "org.mpris.MediaPlayer2.Player",
              method)
        .unwrap()
        .and_then(|status: String| {
            println!("update_playback_future -> status = {}", status);
            Ok(())
        })
}
