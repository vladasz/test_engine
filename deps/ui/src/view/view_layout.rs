use crate::{view::view_internal::ViewInternal, View, ViewSubviews};

pub trait ViewLayout {
    fn calculate_frames(&mut self);
    fn layout(&mut self)
    where
        Self: View,
    {
        self.place.layout();
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn calculate_frames(&mut self) {
        self.absolute_frame = self.frame;
        let orig = self.super_absolute_frame().origin;
        self.absolute_frame.origin += orig;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }
}
