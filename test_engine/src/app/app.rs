#[cfg(mobile)]
use core::ffi::{c_float, c_int};
use std::path::PathBuf;

use gm::flat::Size;
#[cfg(desktop)]
use ui::refs::set_current_thread_as_main;
use ui::{refs::Own, View};

use crate::AppCore;

pub trait App {
    fn setup()
    where Self: Sized {
    }

    fn screen_size() -> Size
    where Self: Sized;

    fn make_root_view() -> Own<dyn View>
    where Self: Sized;

    fn with_core(core: AppCore) -> Self
    where Self: Sized;

    fn core(&mut self) -> &mut AppCore;

    fn assets_path() -> PathBuf
    where Self: Sized {
        PathBuf::new()
    }

    #[cfg(desktop)]
    fn make_core() -> AppCore
    where Self: Sized {
        Self::setup();
        trace!("App setup: OK");
        trace!("Make core");
        set_current_thread_as_main();
        trace!("Marked thread {} as main", ui::refs::current_thread_id());
        let core = AppCore::new(Self::screen_size(), Self::assets_path(), Self::make_root_view());
        trace!("AppCore: OK");
        core
    }

    #[cfg(desktop)]
    fn launch(&mut self) -> u8 {
        trace!("Launch");
        self.core().screen.start_main_loop()
    }
}

pub trait MakeApp {
    #[cfg(desktop)]
    fn make_app() -> Self;

    #[cfg(mobile)]
    fn make_app(
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) -> Box<Self>;
}

impl<T: App> MakeApp for T {
    #[cfg(desktop)]
    fn make_app() -> Self {
        T::with_core(T::make_core())
    }

    #[cfg(mobile)]
    fn make_app(
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) -> Box<Self> {
        T::setup();
        let core = AppCore::new(
            ppi,
            scale,
            refresh_rate,
            resolution_x,
            resolution_y,
            width,
            height,
            diagonal,
            Self::make_root_view(),
        );
        let app = T::with_core(core);
        Box::new(app)
    }
}
