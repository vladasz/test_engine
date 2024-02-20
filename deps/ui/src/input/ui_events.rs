use std::cell::RefCell;

use gm::{flat::Size, volume::GyroData};
use refs::MainLock;
use vents::Event;

use crate::{Touch, WeakView};

static UI_EVENTS: MainLock<UIEvents> = MainLock::new();

#[derive(Default)]
pub struct UIEvents {
    pub scroll: Event<f32>,
    pub gyro_changed: Event<GyroData>,
    pub on_touch: Event<Touch>,
    pub size_changed: Event<Size<u32>>,
    selected_view: RefCell<WeakView>,
}

impl UIEvents {
    pub(crate) fn get() -> &'static Self {
        &UI_EVENTS
    }

    pub fn on_touch() -> &'static Event<Touch> {
        &Self::get().on_touch
    }

    pub fn size_changed() -> &'static Event<Size<u32>> {
        &Self::get().size_changed
    }
}

impl UIEvents {
    pub fn unselect_view(&self) {
        let mut selected_view = self.selected_view.borrow_mut();
        if selected_view.is_null() {
            return;
        }
        selected_view.on_selection_changed(false);
        *selected_view = Default::default();
    }

    pub fn set_selected(&self, mut view: WeakView, selected: bool) {
        let mut selected_view = self.selected_view.borrow_mut();

        if let Some(selected) = selected_view.get() {
            selected.on_selection_changed(false);
            *selected_view = Default::default();
        }

        if selected {
            *selected_view = view;
        }

        view.on_selection_changed(selected);
    }
}
