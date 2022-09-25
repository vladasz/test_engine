use rtools::{Rglica, ToRglica};

use crate::{get_ui_drawer, layout::Placer, SubView, View};

pub trait ViewSubviews {
    fn superview(&self) -> Rglica<dyn View>;
    fn subviews(&self) -> &[Box<dyn View>];
    fn subviews_mut(&mut self) -> &mut [Box<dyn View>];
    fn remove_from_superview(&mut self);
    fn remove_subview_at(&mut self, index: usize) -> Box<dyn View>;
    fn remove_all_subviews(&mut self);

    fn initialize_view<V: 'static + View + Default>(&mut self) -> SubView<V>;
    fn add_subview(&mut self, view: Box<dyn View>) -> Rglica<dyn View>;
}

impl<T: ?Sized + View> ViewSubviews for T {
    fn superview(&self) -> Rglica<dyn View> {
        self.superview
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        &self.subviews
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.subviews
    }

    fn remove_from_superview(&mut self) {
        get_ui_drawer().schedule_remove(self.rglica())
    }

    fn remove_subview_at(&mut self, index: usize) -> Box<dyn View> {
        self.subviews.remove(index)
    }

    fn remove_all_subviews(&mut self) {
        for view in &self.subviews {
            get_ui_drawer().schedule_remove(view.rglica())
        }
    }

    fn initialize_view<V: 'static + View + Default>(&mut self) -> SubView<V> {
        let view = Box::<V>::default();
        let result = view.to_rglica();
        self.add_subview(view);
        result.into()
    }

    fn add_subview(&mut self, mut view: Box<dyn View>) -> Rglica<dyn View> {
        view.superview = self.rglica();
        view.place = Placer::new(view.rglica()).into();
        view.init_views();
        view.setup();
        let res = view.to_rglica();
        self.subviews.push(view);
        res
    }
}
