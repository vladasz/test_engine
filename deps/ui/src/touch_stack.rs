use std::sync::{Mutex, MutexGuard, OnceLock};

use nonempty::NonEmpty;

use crate::{
    touch_layer::TouchLayer,
    view::{ViewData, ViewSubviews},
    UIManager, WeakView,
};

static STACK: OnceLock<Mutex<TouchStack>> = OnceLock::new();

pub struct TouchStack {
    stack: NonEmpty<TouchLayer>,
}

impl TouchStack {
    fn init() -> Mutex<Self> {
        Self {
            stack: NonEmpty::new(UIManager::get().root_view.weak_view().into()),
        }
        .into()
    }

    fn get() -> MutexGuard<'static, Self> {
        STACK.get_or_init(Self::init).lock().unwrap()
    }
}

impl TouchStack {
    fn layer_for(&mut self, view: WeakView) -> &mut TouchLayer {
        let mut view_stack = vec![];

        view_stack.push(view.label().to_string());

        let mut sup = view.superview();

        while sup.is_ok() {
            view_stack.push(sup.label().to_string());
            sup = sup.superview();
        }

        while self.stack.last().root.is_null() {
            self.stack.pop();
        }

        for layer in self.stack.iter_mut().rev() {
            for label in &view_stack {
                if layer.root_name() == *label {
                    return layer.clear_freed();
                }
            }
        }

        unreachable!("Failed to found view touch layer")
    }
}

impl TouchStack {
    pub fn touch_views() -> Vec<WeakView> {
        Self::get().stack.last().views()
    }

    pub fn enable_for(view: WeakView, priority: bool) {
        Self::get().layer_for(view).add(view, priority)
    }

    pub fn disable_for(view: WeakView) {
        Self::get().layer_for(view).remove(view)
    }

    pub fn push_layer(view: WeakView) {
        Self::get().stack.push(view.into())
    }

    pub fn pop_layer(view: WeakView) {
        let pop = Self::get().stack.pop().unwrap();
        assert_eq!(
            pop.root_addr(),
            view.addr(),
            "Inconsistent pop_touch_view call. Expected: {} got: {}",
            pop.root_name(),
            view.label()
        );
    }

    pub fn root_name() -> String {
        Self::get().stack.last().root_name().to_string()
    }

    pub fn dump() -> Vec<Vec<String>> {
        let mut result = vec![];

        for layer in &Self::get().stack {
            let mut layer_vec = vec![];

            layer_vec.push(format!("Layer: {}", layer.root_name()));

            for view in layer.views() {
                if view.is_null() {
                    continue;
                }
                layer_vec.push(format!("View: {}", view.label()));
            }

            result.push(layer_vec);
        }

        result
    }
}
