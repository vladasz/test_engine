use std::{
    ffi::{c_float, c_int},
    marker::PhantomData,
    os::raw::c_ulong,
};

use gl_wrapper::monitor::Monitor;
use tokio::runtime::Runtime;
use ui::{input::touch::Event, Touch};

use crate::{game_view::GameView, Screen};

pub struct App<T> {
    pub screen:  Option<Screen>,
    pub monitor: Monitor,
    runtime:     Runtime,
    _view:       PhantomData<T>,
}

impl<T: GameView + 'static> App<T> {
    pub fn create_screen(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            let mut screen = Screen::new((width, height).into());

            screen.ui.set_view(T::boxed());
            screen.ui.add_debug_view();

            screen.add_monitor(self.monitor.clone());

            self.screen = screen.into();
        });
    }

    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            self.screen.as_mut().unwrap().set_size((width, height).into());
        });
    }

    pub fn update_screen(&mut self) {
        self.runtime
            .block_on(async { self.screen.as_mut().unwrap().update() });
    }

    pub fn on_touch(&mut self, id: c_ulong, x: c_float, y: c_float, event: c_int) {
        #[allow(clippy::useless_conversion)]
        self.runtime.block_on(async {
            self.screen.as_mut().unwrap().ui.on_touch(Touch {
                id:       id.into(),
                position: (x, y).into(),
                event:    Event::from_int(event),
            })
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

        self.monitor = monitor;
    }
}

impl<T: GameView> Default for App<T> {
    fn default() -> Self {
        Self {
            screen:  Default::default(),
            monitor: Default::default(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            _view:   Default::default(),
        }
    }
}
