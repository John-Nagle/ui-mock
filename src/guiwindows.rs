/// #  guiwindows.rs -- window and menu layout.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  Animats
//  June 2022
//
use std::path::PathBuf;
use crate::t;
use crate::{LoginParams, GridSelectParams, CommonState};

/// User events sent to the main event loop
#[derive(Debug)]
pub enum GuiEvent {
    Startup,                                        // back to startup state
    OpenReplay(Option<PathBuf>),                    // open a replay file
    SaveReplay(PathBuf),                            // save into a replay file
    LoginTo(GridSelectParams),                      // ask for login params
    LoginStart(LoginParams),                        // start the login process
    ////Login(ConnectInfo),                         // login dialog result
    ErrorMessage((String, Vec<String>)),            // pops up an warning dialog (title, [text])
    LogMessage(String),                             // log to GUI
    Shutdown                                        // shut down and exit
}

/// GUI states.
//  The main states of the system.
//  This is a state machine
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SystemMode {
    Startup,  // idle, waiting for grid selection
            // -> Login, Replay. Exit
    Login,  // login dialog is up.
            // -> Connected, Start
    Connecting, // Connecting to server
            // -> Connected, Start
    Connected, // Fully connected, all menus live
            // -> Shutdown
    Replay, // in replay mode, some menus live
            // -> Shutdown
    Shutdown, // shutting down and cleaning up
            // -> Exit, Start
}


/// Pick replay file, async form
#[cfg (feature="replay")]
pub fn pick_replay_file_async(state: &mut CommonState, window: &winit::window::Window) {
    fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
        // this is stupid... use any executor of your choice instead
        std::thread::spawn(move || futures::executor::block_on(f));
    }

    let channel = state.get_send_channel().clone(); // save send channel
    //  Pop up the file dialog
    let task = rfd::AsyncFileDialog::new()
        .set_title(t!("title.open_replay", state.get_lang()))
        .add_filter("json", &["json"])
        .set_parent(window)
        .pick_file();
    // Await somewhere else
    execute(async move {
        let file = task.await;              // wait for dialog completion
        let replay_path_opt = if let Some(file) = file {
            // If you are on native platform you can just get the path
            println!("{:?}", file.path());

            // If you care about wasm support you just read() the file
            ////file.read().await;
            log::warn!("File picked: {:?}", file.path());
            Some(file.path().to_path_buf())
        } else {
            None
        };
        //  Send dialog result to the main event loop for action.
        let _ = CommonState::send_gui_event_on_channel(&channel, GuiEvent::OpenReplay(replay_path_opt)); // if we can't send, we must be shutting down
    });
}



