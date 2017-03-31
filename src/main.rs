#![feature(proc_macro, conservative_impl_trait)]

extern crate dbus;
extern crate gdk;
extern crate gtk;
extern crate futures;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate tokio_core;

use std::time::Duration;

mod player;

use player::{update_playback_future,
             get_playback_status, get_playback_metadata,
             PlaybackMetadata};

use gtk::{
    BoxExt,
    Button,
    ButtonExt,
    Inhibit,
    Label,
    OrientableExt,
    // ProgressBar,
    WidgetExt,
    Window,
    WindowExt
};

use gtk::Orientation::Vertical;

use relm::{Relm, RemoteRelm};
use relm_attributes::widget;

use tokio_core::reactor::Interval;


#[derive(Clone, Debug)]
struct Model {
    status: String,
    metadata: PlaybackMetadata,
    progress: f64
}

#[derive(Msg, Debug)]
enum Msg {
    Previous,
    PlayPause,
    Stop,
    Next,
    UpdatePlaybackStatus(()),
    UpdateMetadata(()),
    Quit
}

#[widget]
impl relm::Widget<Msg> for Win {
    fn model() -> Model {
        Model {
            status: get_playback_status(),
            metadata: get_playback_metadata(),
            progress: 0f64
        }
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::UpdateMetadata(()) => {
                &model.metadata = get_playback_metadata();
            },
            Msg::UpdatePlaybackStatus(()) => {
                &model.status = get_playback_status();
            },
            _ => ()
        }
    }

    fn update_command(relm: &Relm<Msg>, event: Msg, _: &mut Model) {
        match event {
            Msg::Previous => {
                let status_future = update_playback_future("Previous", relm.handle());
                relm.connect_exec_ignore_err(status_future, Msg::UpdatePlaybackStatus);
            },
            Msg::PlayPause => {
                let status_future = update_playback_future("PlayPause", relm.handle());
                relm.connect_exec_ignore_err(status_future, Msg::UpdatePlaybackStatus);
            },
            Msg::Stop => {
                let status_future = update_playback_future("Stop", relm.handle());
                relm.connect_exec_ignore_err(status_future, Msg::UpdatePlaybackStatus);
            },
            Msg::Next => {
                let status_future = update_playback_future("Next", relm.handle());
                relm.connect_exec_ignore_err(status_future, Msg::UpdatePlaybackStatus);
            },
            _ => ()
        }
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let status_timer = Interval::new(Duration::from_millis(350), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(status_timer, Msg::UpdatePlaybackStatus);

        let metadata_timer = Interval::new(Duration::from_millis(350), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(metadata_timer, Msg::UpdateMetadata);
    }

    view! {
        Window(gtk::WindowType::Popup) {
            title: "Rusty Sounds",
            gtk::Box {
                orientation: Vertical,
                gtk::Box {
                    spacing: 2,
                    Button {
                        label: "Prev",
                        clicked => Msg::Previous
                    },
                    Button {
                        label: if model.status == "Playing".to_string() { "Pause" } else { "Play" },
                        clicked => Msg::PlayPause
                    },
                    Button {
                        label: "Stop",
                        sensitive: if model.status == "Playing".to_string() { true } else { false },
                        clicked => Msg::Stop
                    },
                    Button {
                        label: "Next",
                        clicked => Msg::Next
                    },
                    // ProgressBar {
                    //     show_text: true,
                    //     fraction: model.progress
                    // }
                },
                gtk::Box {
                    spacing: 2,
                    Label {
                        text: model.status.as_str()
                    },
                    Label {
                        text: model.metadata.artist.as_str()
                    },
                    Label {
                        text: model.metadata.album.as_str()
                    },
                    Label {
                        text: model.metadata.title.as_str()
                    },
                }
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false))
        }
    }
}

fn main() {
    Relm::run::<Win>().unwrap();
}
