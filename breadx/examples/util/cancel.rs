//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! Utilities for canceling the programs.
//!
//! This is used in CI to be able to run these programs as a testing
//! measure. The ideal case opens a new display and sends the kill
//! command to the given root window. The less-ideal case, in case
//! a deadlock happens, manually kills the program after one minute.
//!
//! These functions are only active if the "BREADX_EXAMPLE_TIMEOUT"
//! environment variable is present at compile time.

#![cfg(feature = "std")]

use breadx::{display::DisplayConnection, prelude::*, protocol::xproto};
use std::{process, thread, time::Duration};

const SKIP_TIMEOUT: bool = option_env!("BREADX_EXAMPLE_TIMEOUT").is_none();

/// Spawns the thread that kills the entire program.
pub fn spawn_kill_thread() {
    if SKIP_TIMEOUT {
        return;
    }

    thread::Builder::new()
        .name("kill-thread".to_string())
        .spawn(|| {
            // wait one minute
            thread::sleep(Duration::from_secs(60));

            // kill the program
            tracing::error!("{}", KILL_PROGRAM);

            process::exit(0);
        })
        .expect("failed to spawn kill thread");
}

/// Connect to the server and send a close signal to the main window
/// after the given amount of time.
pub fn spawn_close_thread(main_window: xproto::Window) {
    if SKIP_TIMEOUT {
        return;
    }

    thread::Builder::new()
        .name("close-thread".to_string())
        .spawn(move || {
            // wait one second
            thread::sleep(Duration::from_secs(1));

            tracing::warn!("{}", CLOSE_PROGRAM);

            // open up a new connection
            let mut display = DisplayConnection::connect(None).unwrap();

            // intern the needed atoms
            let protocols = display.intern_atom(false, "WM_PROTOCOLS").unwrap();
            let delete_window = display.intern_atom(false, "WM_DELETE_WINDOW").unwrap();
            let protocols = display.wait_for_reply(protocols).unwrap().atom;
            let delete_window = display.wait_for_reply(delete_window).unwrap().atom;

            // create the event
            let event = xproto::ClientMessageEvent::new(
                32,
                main_window,
                protocols,
                [delete_window, 0, 0, 0, 0],
            );
            let mut send_event = xproto::SendEventRequest {
                propagate: false,
                destination: main_window,
                event_mask: xproto::EventMask::NO_EVENT.into(),
                event: std::borrow::Cow::Owned(event.into()),
            };

            // send the events, fallibly
            display.send_void_request(send_event.clone(), true).ok();
            send_event.event_mask = xproto::EventMask::SUBSTRUCTURE_REDIRECT.into();
            display.send_void_request(send_event, true).ok();
            display.flush().unwrap();
        })
        .expect("failed to spawn close thread");
}

const KILL_PROGRAM: &str = "
The program is not responding to the close request being sent
to the window. The program will now exit with an error status,
in order to prevent the CI from being deadlocked.
";

const CLOSE_PROGRAM: &str = "
The program will now send a close request to the main window,
in order to stop the event loop. This will shut down the program
safely.

If this is not desired behavior, disable the `BREADX_EXAMPLE_TIMEOUT`
environment variable.
";
