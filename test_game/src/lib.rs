use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use gl_wrapper::monitor::Monitor;
use test_engine::{
    gm::Size,
    ui::{input::touch::Event, Touch},
    Screen,
};
use tools::Boxed;

use crate::test_view::TestGameView;

mod test_level;
mod test_view;

#[cfg(target_os = "android")]
#[macro_use]
extern crate log;

static mut SCREEN: *mut Screen = ptr::null_mut();
static mut MONITOR: *mut Monitor = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        SCREEN = Box::into_raw(Box::new(
            Screen::new(Default::default())
                .set_view(TestGameView::boxed())
                .add_debug_view(),
        ));

        SCREEN.as_mut().unwrap().add_monitor(MONITOR.as_ref().unwrap().clone());
    }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_float, height: c_float) {
    unsafe {
        SCREEN.as_mut().unwrap().set_size(Size { width, height });
    }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe {
        SCREEN.as_mut().unwrap().update();
    }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    #[allow(clippy::useless_conversion)]
    unsafe {
        SCREEN.as_mut().unwrap().on_touch(Touch {
            id:       id.into(),
            position: (x * 2.0, y * 2.0).into(),
            event:    Event::from_int(event),
        })
    }
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate android_logger;

    use android_logger::{Config, FilterBuilder};
    use gl_wrapper::monitor::Monitor;
    use log::Level;

    fn setup_logger() {
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace)
                .with_tag("test_engine")
                .with_filter(
                    FilterBuilder::new()
                        .parse("debug,hello::crate=error")
                        .build(),
                ),
        );

        trace!("this is a verbose {}", "message");
        error!("setup_logger");
    }

    use android_ndk_sys::{jclass, jobject, JNIEnv};

    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setup(_: JNIEnv, _: jclass) {
        create_screen();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setAssetManager(
        env: JNIEnv,
        _: jclass,
        asset_manager: jobject,
    ) {
        setup_logger();
        tools::file::set_asset_manager(env, asset_manager);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setScreenSize(
        _: JNIEnv,
        _: jclass,
        width: c_int,
        height: c_int,
    ) {
        set_screen_size(width as _, height as _);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_update(_: JNIEnv, _: jclass) {
        update_screen();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_onTouch(
        _: JNIEnv,
        _: jclass,
        id: c_ulong,
        x: c_float,
        y: c_float,
        event: c_int,
    ) {
        on_touch(id, x, y, event)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setMonitor(
        _: JNIEnv,
        _: jclass,
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolutionX: c_int,
        resolutionY: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) {

        error!("miska");

        let monitor = Monitor::new(
            "Android screen".into(),
            ppi as _,
            scale,
            refresh_rate as _,
            (resolutionX, resolutionY).into(),
            (width, height).into(),
            diagonal as _,
        );

        error!("{:?}", &monitor);

        MONITOR = Box::into_raw(Box::new(monitor));
    }
}
