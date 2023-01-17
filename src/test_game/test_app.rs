use std::ops::{Deref, DerefMut};

use rtools::init_log;
use test_engine::{app_core::AppCore, App};
use ui::refs::{enable_ref_stats_counter, Own};

use crate::benchmark::UIDebugView;

pub struct TestApp {
    app: AppCore,
}

impl TestApp {
    fn setup() {
        enable_ref_stats_counter(true);
        init_log(false, 4);
    }

    fn make_root_view() -> Own<UIDebugView> {
        Default::default()
    }
}

#[cfg(desktop)]
mod desktop {
    use test_engine::{app_core::AppCore, paths::home};

    use crate::test_game::TestApp;

    impl TestApp {
        pub fn launch(&mut self) {
            self.app.screen.start_main_loop();
        }
    }

    impl Default for TestApp {
        fn default() -> Self {
            Self::setup();
            let app = AppCore::new((1000, 600), home().join("test_engine"), Self::make_root_view());
            Self { app }
        }
    }
}

impl App for TestApp {}

impl Deref for TestApp {
    type Target = AppCore;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for TestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

#[cfg(mobile)]
pub mod mobile {
    use std::ffi::{c_float, c_int};

    use test_engine::app_core::AppCore;

    use crate::test_game::TestApp;

    impl TestApp {
        pub fn new(
            ppi: c_int,
            scale: c_float,
            refresh_rate: c_int,
            resolution_x: c_int,
            resolution_y: c_int,
            width: c_float,
            height: c_float,
            diagonal: c_float,
        ) -> Box<Self> {
            Self::setup();

            let app = AppCore::new(
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
            Box::new(Self { app })
        }
    }
}
