use anyhow::Result;
use dispatch::from_main;
use gm::{Color, U8Color, flat::Point};
use ui::{
    Button, Container, HighlightView, Setup, UIManager, View, ViewData, ViewFrame, ViewSubviews, WeakView,
};

use crate::{App, gm::Apply, ui::Screenshot};

pub fn add_corners(mut view: WeakView, color: Color) {
    let v1 = view.add_view::<Container>();
    let v2 = view.add_view::<Container>();
    let v3 = view.add_view::<Container>();
    let v4 = view.add_view::<Container>();

    [v1, v2, v3, v4].apply(|mut a| {
        a.place().size(100, 100);
        a.set_color(color);
    });

    v1.place().tl(0);
    v2.place().tr(0);
    v3.place().bl(0);
    v4.place().br(0);
}

#[allow(dead_code)]
pub fn add_action(action: impl FnMut() + Send + 'static) {
    let mut button = UIManager::root_view_weak()
        .__add_subview_internal(Button::new(), true)
        .downcast::<Button>()
        .unwrap();
    button.place().size(100, 100).bl(0);
    button.set_color(Color::LIGHT_GRAY);
    button.on_tap(action);
    button.base_view_mut().view_label = "Debug Action Button".into();
}

pub async fn check_colors(data: &str) -> Result<()> {
    let screenshot = App::take_screenshot().await?;

    let lines: Vec<_> = data.split('\n').collect();

    for line in lines {
        let parts: Vec<_> = line.split('-').collect();

        if parts.len() != 2 {
            continue;
        }

        let pos = parts[0];
        let color = parts[1];

        let pos: Vec<_> = pos.split(' ').filter(|a| !a.is_empty()).collect();
        let color: Vec<_> = color.split(' ').filter(|a| !a.is_empty()).collect();

        let pos: Point = Point::new(pos[0].parse().unwrap(), pos[1].parse().unwrap());
        let color: U8Color = U8Color::rgba(
            color[0].parse().unwrap(),
            color[1].parse().unwrap(),
            color[2].parse().unwrap(),
            255,
        );

        check_pixel_color(&screenshot, pos, color).await;
    }

    Ok(())
}

pub async fn check_pixel_color(screenshot: &Screenshot, pos: Point, color: U8Color) {
    let pixel: U8Color = screenshot.get_pixel(pos);
    let pixel_f32: Color<f32> = pixel.into();
    let color_f32: Color<f32> = color.into();

    let diff = pixel_f32.diff(color_f32);

    let max_diff = 0.1;

    if diff > max_diff {
        from_main(move || {
            let mut high = HighlightView::new();
            high.set_z_position(0.1);

            UIManager::root_view_weak()
                .__add_subview_internal(high, true)
                .downcast_view::<HighlightView>()
                .unwrap()
                .set(pos, color_f32, pixel_f32);
        })
        .await;
    }

    assert!(
        diff <= max_diff,
        "Color diff is too big: {diff}. Max: {max_diff}. Position: {pos:?}. \nExpected: {color}, got: \
         {pixel}"
    );
}
