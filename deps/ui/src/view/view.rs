use refs::{AsAny, Weak};

use crate::{ViewBase, ViewCallbacks, view::view_callbacks::ViewInternalSetup};

pub trait View: ViewCallbacks + ViewInternalSetup + AsAny {
    fn init_views(&mut self);
    fn base_view(&self) -> &ViewBase;
    fn base_view_mut(&mut self) -> &mut ViewBase;
    fn weak_view(&self) -> WeakView;
}

pub type WeakView = Weak<dyn View>;
