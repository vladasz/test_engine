use serde::{Deserialize, Serialize};
use test_engine::{
    audio::Sound,
    main_view::{HasLevel, MainView},
    net::{GetRequest, API},
    rtools::{
        data_manager::{DataManager, Handle},
        Rglica, ToRglica,
    },
    sprite_view::SpriteView,
    sprites::{Control, Player},
    ui::{
        basic::Button,
        complex::{AnalogStickView, LabeledSlider},
        impl_view,
        layout::Anchor,
        test::test_view::TestView,
        view, DPadView, View, ViewBase, ViewCallbacks, ViewFrame, ViewSubviews,
    },
    ui_layer::UILayer,
    Image, Level,
};

use crate::{test_game::test_game_level::TestGameLevel, BenchmarkView, UITestView};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

const API: API = API::new("ec2-18-217-89-172.us-east-2.compute.amazonaws.com");

const GET_USERS: GetRequest<Vec<User>> = API.get("get_users");

#[view]
#[derive(Default, Debug)]
pub struct TestGameView {
    level:       TestGameLevel,
    dpad:        Rglica<DPadView>,
    left_stick:  Rglica<AnalogStickView>,
    sprite_view: Rglica<SpriteView>,
    test_view:   Rglica<TestView>,

    ui_scale_slider:   Rglica<LabeledSlider>,
    game_scale_slider: Rglica<LabeledSlider>,

    to_benchmark: Rglica<Button>,
    to_test:      Rglica<Button>,

    play:       Rglica<Button>,
    sound:      Handle<Sound>,
    async_task: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl_view!(TestGameView);

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        self.dpad
            .on_press
            .set(&self.level.player, |player, dir| player.move_by_direction(dir));

        self.left_stick.on_change.set(&self.level.player, |player, dir| {
            player.add_impulse(dir);
        });
    }

    fn setup_sliders(&mut self) {
        self.game_scale_slider = self.add_view_with_frame((50, 280));
        self.game_scale_slider.set_start(0.5).set_finish(10);
        self.game_scale_slider.on_change.set(self, |this, scale| {
            this.level().drawer_mut().set_scale(scale);
        });

        self.ui_scale_slider = self.add_view_with_frame((50, 280));
        self.ui_scale_slider.set_start(0.2).set_finish(4);
        self.ui_scale_slider.on_change.set(self, |this, scale| {
            this.ui.set_scale(scale);
        });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500));

        self.sprite_view = self.add_view_with_frame((500, 180));

        self.level
            .base()
            .on_sprite_selected
            .set(self, |this, sprite| this.sprite_view.set_sprite(sprite));

        self.dpad = self.add_view();
        self.dpad
            .set_images(
                Image::get("up.png"),
                Image::get("down.png"),
                Image::get("left.png"),
                Image::get("right.png"),
            )
            .make_layout(|l| {
                l.width(200).height(150);
                l.bottom().left().offset(5);
            });

        self.left_stick = self.add_view();

        self.setup_sliders();

        self.test_view = self.add_view_with_frame((280, 400));
        self.test_view
            .set_image(Image::get("cat.png"))
            .set_button_image(Image::get("square.png"))
            .set_animation_image(Image::get("palm.png"));

        self.to_benchmark = self.add_view();
        self.to_benchmark.set_text("Benchmark").set_frame((120, 20));
        self.to_benchmark
            .on_tap
            .set(self, |this, _| this.ui.set_view::<BenchmarkView>());

        self.to_test = self.add_view();
        self.to_test.set_text("Test").set_frame((120, 20));
        self.to_test
            .on_tap
            .set(self, |this, _| this.ui.set_view::<UITestView>());

        self.play = self.add_view();
        self.play.set_text("Play sound").set_frame((120, 20));
        self.play.on_tap.set(self, |this, _| this.sound.play());

        self.sound = Sound::get("retro.wav");

        self.async_task = self.add_view();
        self.async_task.set_text("Async task").set_frame((120, 20));
        self.async_task.on_tap.set(self, |this, _| {
            GET_USERS.get(this, |this, error, result| {
                if let Some(error) = error {
                    dbg!(error);
                    return;
                }

                dbg!(&result);

                this.async_task.set_text(result.first().unwrap().login.clone());
            });
        });
    }
}

impl ViewCallbacks for TestGameView {
    fn setup(&mut self) {
        self.setup_ui();
        self.setup_level();
    }

    fn layout(&mut self) {
        self.left_stick
            .deprecated_place()
            .anchor(self.dpad, Anchor::Right, Anchor::Bot, 20);

        self.game_scale_slider
            .deprecated_place()
            .proportional_height(0.5)
            .anchor(self.dpad, Anchor::Top, Anchor::Left, 10);

        self.ui_scale_slider
            .deprecated_place()
            .proportional_height(0.5)
            .anchor(self.game_scale_slider, Anchor::Right, Anchor::Center, 10);

        self.sprite_view
            .deprecated_place()
            .anchor(self.ui_scale_slider, Anchor::Right, Anchor::Bot, 10);

        self.test_view
            .deprecated_place()
            .bottom_right(20)
            .proportional_width(0.28)
            .proportional_height(0.8);

        self.to_benchmark.deprecated_place().bottom_center(20);

        self.to_test
            .deprecated_place()
            .anchor(self.to_benchmark, Anchor::Top, Anchor::Center, 10);

        self.play
            .deprecated_place()
            .anchor(self.to_test, Anchor::Top, Anchor::Center, 10);

        self.async_task
            .deprecated_place()
            .anchor(self.play, Anchor::Top, Anchor::Center, 10);
    }
}

impl MainView for TestGameView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}

impl HasLevel for TestGameView {
    fn player(&self) -> Rglica<Player> {
        self.level.player
    }

    fn level(&self) -> Rglica<dyn Level> {
        (&self.level as &dyn Level).to_rglica()
    }
}
