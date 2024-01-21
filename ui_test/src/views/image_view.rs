use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    gm::{flat::Point, Color},
    manage::data_manager::DataManager,
    Image, Screen,
};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewSetup, ViewTouch};
use ui_views::ImageView;

#[view]
struct ImageTestView {
    image_view: SubView<ImageView>,
}

impl ViewSetup for ImageTestView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch.began.val(|touch| {
            let color = Screen::read_pixel(touch.position);

            println!(
                "(({}, {}), ({}, {}, {}, {})),",
                touch.position.x, touch.position.y, color.r, color.g, color.b, color.a
            );
        });

        // SystemEvents::get().after_draw.sub(move || {
        //     let pos = UILayer::get().cursor_position;
        //     let color = Screen::read_pixel(pos);
        //     self.button.set_color(color);
        // });

        self.image_view.place.center().relative(Anchor::Size, self, 0.5);
        self.image_view.image = Image::get("blue.png");
    }
}

async fn check_pixel_color(pos: Point, color: Color) {
    from_main(move || {
        assert!(Screen::read_pixel(pos).diff(color) < 0.012);
    })
    .await
}

async fn check_colors<const N: usize>(data: [((f32, f32), (f32, f32, f32, f32)); N]) {
    for val in data {
        check_pixel_color(
            (val.0 .0, val.0 .1).into(),
            Color::rgba(val.1 .0, val.1 .1, val.1 .2, val.1 .3),
        )
        .await
    }
}

pub async fn test_image_view() -> Result<()> {
    Screen::set_test_view::<ImageTestView>(400, 400).await;

    check_colors([
        ((98.0, 113.0), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((105.0, 118.0), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((295.0, 119.0), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((304.0, 121.0), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((95.62109, 205.23438), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((106.51172, 206.72266), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((140.4336, 208.8789), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((149.53516, 208.23438), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((253.94531, 212.3125), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((261.4414, 212.63672), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((293.48047, 212.85156), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((303.66406, 214.58594), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((201.14063, 97.015625), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((200.36719, 106.41016), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((201.94531, 130.1289), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((203.03125, 143.53125), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((202.26563, 251.76563), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((201.08594, 263.34766), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((199.54297, 290.25), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((203.57813, 309.5586), (0.5019608, 0.5019608, 0.5019608, 1.0)),
    ])
    .await;

    from_main(|| {
        Screen::current().set_size((200, 600));
    })
    .await;

    check_colors([
        ((43.64453, 260.22266), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((53.9375, 259.70313), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((144.4414, 264.0664), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((159.22266, 263.51172), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((43.273438, 340.6875), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((57.30078, 341.5586), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((140.8125, 343.5078), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((167.71094, 343.67188), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((158.82031, 320.79688), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((138.65625, 320.6172), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((126.859375, 321.23047), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((78.91016, 321.83203), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((68.10156, 322.01172), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((41.128906, 322.70313), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((103.51953, 243.35938), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((104.78125, 256.73438), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((106.44531, 274.11328), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((103.94141, 316.91797), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((103.93359, 336.6953), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((104.29297, 364.83203), (0.5019608, 0.5019608, 0.5019608, 1.0)),
    ])
    .await;

    from_main(|| {
        Screen::current().set_size((600, 250));
    })
    .await;

    check_colors([
        ((135.64063, 18.582031), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((324.05078, 20.738281), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((487.65625, 29.390625), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((488.20313, 116.25), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((423.26172, 119.796875), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((290.97656, 122.41797), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((174.45313, 128.72656), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((58.601563, 128.73828), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((126.52734, 234.625), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((191.41016, 234.29688), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((357.09766, 226.97266), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((519.58203, 221.97266), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((374.1836, 198.58984), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((234.92969, 196.16797), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((231.72266, 48.242188), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((365.90234, 40.652344), (0.5019608, 0.5019608, 0.5019608, 1.0)),
    ])
    .await;

    debug!("Image view test: OK");

    Ok(())
}
