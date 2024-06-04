#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::env::var;

use anyhow::Result;
use log::info;
use test_engine::{
    ui::{Container, ViewSetup},
    App,
};
use views::complex::keyboard_view_test::test_keyboard_view;

use crate::{
    base::{
        keymap::test_keymap, layout::test_layout, modal_test::test_modal, on_tap_add::test_add_on_tap,
        out_bounds_test::test_out_bounds, present_test::test_present, selection::test_selection,
        template::test_template, text_occlusion::test_text_occlusion, touch_order::test_touch_order,
        touch_stack::test_touch_stack, view_order::test_view_order,
    },
    views::{
        alert::test_alert,
        basic::{
            button::test_button, image_view::test_image_view, inject_touch::test_inject_touch,
            label::test_label, multiline_label::test_multiline, scroll_view::test_scroll_view,
            slider::test_slider, stick::test_stick, switch::test_switch, text_field::test_text_field,
        },
        complex::{
            buttons_on_table::test_buttons_on_table_view, collection_view::test_collection_view,
            drop_down::test_drop_down, form::test_form_view, highlight::test_highlight,
            number_view::test_number_view, table_view::test_table_view,
        },
        point_view::test_point_view,
        render_image_path::test_render_image_path,
    },
};

mod base;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            test().await?;
            info!("Cycle {i}: OK");
        }

        _ = skip();

        App::stop();

        Ok(())
    })
    .await
}

async fn test() -> Result<()> {
    test_keyboard_view().await?;
    test_out_bounds().await?;
    test_number_view().await?;
    test_form_view().await?;
    test_inject_touch().await?;
    test_highlight().await?;
    test_label().await?;
    test_scroll_view().await?;
    test_table_view().await?;
    test_modal().await?;
    test_view_order().await?;
    test_collection_view().await?;
    test_slider().await?;
    test_drop_down().await?;
    test_add_on_tap().await?;
    test_buttons_on_table_view().await?;
    test_touch_order().await?;
    test_template().await?;
    test_present().await?;
    test_stick().await?;
    test_point_view().await?;
    test_touch_stack().await?;
    test_text_occlusion().await?;
    test_text_field().await?;
    test_selection().await?;
    test_keymap().await?;
    test_image_view().await?;
    test_button().await?;
    test_switch().await?;
    test_alert().await?;
    test_multiline().await?;

    Ok(())
}

async fn skip() -> Result<()> {
    test_render_image_path().await?;
    test_layout().await?;

    Ok(())
}
