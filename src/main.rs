#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

//! Sacred Geometry for Precursor. The constructions live in `sacred-core`
//! (host-tested). This file routes messages; nothing animates,
//! so there is no pump: the app is idle between key presses.

mod app;

use num_traits::{FromPrimitive, ToPrimitive};
use precursor_appkit::{app as kit, keys};

/// xous-names server name. Any unique string; NOT the GAM app name.
const SERVER_NAME: &str = "_Sacred Geometry_";

#[derive(Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub(crate) enum AppOp {
    Redraw = 0,
    Rawkeys,
    FocusChange,
    Quit,
}

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("sacred PID is {}", xous::process::id());

    let xns = xous_names::XousNames::new().unwrap();
    let sid = xns.register_name(SERVER_NAME, None).expect("can't register server");

    // GAM app name comes from apps/manifest.json key "sacred".
    let reg = kit::register(
        &xns,
        sid,
        gam::APP_NAME_SACRED,
        gam::UxType::Framebuffer,
        kit::Opcodes {
            redraw: AppOp::Redraw.to_u32().unwrap(),
            rawkeys: Some(AppOp::Rawkeys.to_u32().unwrap()),
            focus_change: Some(AppOp::FocusChange.to_u32().unwrap()),
            got_input: None,
        },
    );

    let mut app = app::App::new(reg.screen);

    loop {
        let msg = xous::receive_message(sid).unwrap();
        match FromPrimitive::from_usize(msg.body.id()) {
            Some(AppOp::Redraw) => app.draw(),
            Some(AppOp::Rawkeys) => xous::msg_scalar_unpack!(msg, k1, k2, k3, k4, {
                for key in keys::decode_rawkeys([k1, k2, k3, k4]) {
                    if app.on_key(key) == app::Action::Leave {
                        kit::leave(app.screen());
                    }
                }
                app.draw();
            }),
            Some(AppOp::FocusChange) => xous::msg_scalar_unpack!(msg, code, _, _, _, {
                if let gam::FocusState::Foreground = kit::focus_state(code) {
                    app.on_foreground();
                }
            }),
            Some(AppOp::Quit) => break,
            None => log::warn!("unknown opcode {:?}", msg.body.id()),
        }
    }
    kit::shutdown(&xns, sid)
}
