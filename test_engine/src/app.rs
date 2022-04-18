#![allow(clippy::mismatched_target_os)]

use std::{
    ffi::{c_float, c_int},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use cfg_if::cfg_if;
use gl_wrapper::monitor::Monitor;
use gm::volume::GyroData;
use rtools::Unwrap;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use ui::{input::touch::TouchEvent, Touch};

use crate::{game_view::GameView, Screen};

pub struct App<T> {
    pub screen:      Unwrap<Screen>,
    runtime:         Runtime,
    _view:           PhantomData<T>,
    _touch_sender:   UnboundedSender<Touch>,
    _touch_receiver: UnboundedReceiver<Touch>,
}

impl<T: GameView + 'static> App<T> {
    fn create_screen(&mut self, assets_path: &Path, monitor: Monitor) {
        self.runtime.block_on(async {
            let mut screen = Screen::new(assets_path, monitor.resolution);

            screen.ui.set_view(T::boxed());
            screen.ui.add_debug_view();

            screen.add_monitor(monitor);

            self.screen = screen.into();
        });
    }

    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            self.screen.set_size((width, height).into());
        });
    }

    pub fn update_screen(&mut self) {
        self.runtime.block_on(async {
            #[cfg(android)]
            while let Ok(touch) = self._touch_receiver.try_recv() {
                self.screen.ui.on_touch(touch);
            }
            self.screen.update();
        });
    }

    pub fn on_touch(&mut self, id: u64, x: c_float, y: c_float, event: c_int) {
        let touch = Touch {
            id,
            position: (x, y).into(),
            event: TouchEvent::from_int(event),
        };

        cfg_if! { if #[cfg(android)] {
            if let Err(err) = self._touch_sender.send(touch) {
                error!("Error sending touch: {:?}", err);
            }
        } else {
            self.runtime.block_on(async {
                self.screen.ui.on_touch(touch);
            });
        }};
    }

    pub fn set_gyro(&mut self, pitch: c_float, roll: c_float, yaw: c_float) {
        self.runtime.block_on(async {
            self.screen.on_gyro_changed(GyroData { pitch, roll, yaw });
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_monitor(
        &mut self,
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) {
        let monitor = Monitor::new(
            "Phone screen".into(),
            ppi as _,
            scale,
            refresh_rate as _,
            (resolution_x, resolution_y).into(),
            (width, height).into(),
            diagonal as _,
        );

        error!("{:?}", &monitor);
        dbg!(&monitor);

        self.create_screen(&PathBuf::new(), monitor);
    }
}

impl<T: GameView> Default for App<T> {
    fn default() -> Self {
        let (_touch_sender, _touch_receiver) = unbounded_channel::<Touch>();
        Self {
            screen: Default::default(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            _view: Default::default(),
            _touch_sender,
            _touch_receiver,
        }
    }
}
