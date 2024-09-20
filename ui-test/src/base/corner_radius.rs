use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Container, ViewData, ViewSetup, UI},
    ui_test::record_ui_test,
};

#[view]
struct CornerRadiusTestView {
    #[init]
    tall:   Container,
    square: Container,
    wide:   Container,
}

impl ViewSetup for CornerRadiusTestView {
    fn setup(mut self: Weak<Self>) {
        self.square.set_color(Color::BLUE).place().size(100, 100).center();
    }
}

pub async fn test_corner_radius() -> Result<()> {
    UI::init_test_view::<CornerRadiusTestView>().await;

    record_ui_test().await;

    debug!("Corner radius test: OK");

    Ok(())
}
