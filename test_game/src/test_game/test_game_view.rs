use test_engine::{
    audio::Sound,
    game_view::GameView,
    gm::Color,
    rtools::{
        data_manager::{DataManager, Handle},
        Boxed, Rglica, ToRglica,
    },
    sprite_view::SpriteView,
    sprites::Control,
    ui::{
        basic::Button,
        complex::{AnalogStickView, LabeledSlider},
        placer::Anchor,
        test::test_view::TestView,
        view_base::{add_view, add_view_with_frame, make_view_on, ViewBase},
        DPadView, Touch, View,
    },
    ui_layer::UILayer,
    Image, Level,
};

use crate::{test_game::test_game_level::TestGameLevel, BenchmarkView};

#[derive(Default, Debug)]
pub struct TestGameView {
    base:         ViewBase,
    level:        TestGameLevel,
    dpad:         Rglica<DPadView>,
    left_stick:   Rglica<AnalogStickView>,
    sprote_view:  Rglica<SpriteView>,
    scale_slider: Rglica<LabeledSlider>,
    test_view:    Rglica<TestView>,

    to_benchmark: Rglica<Button>,

    play:  Rglica<Button>,
    sound: Handle<Sound>,

    ui: Rglica<UILayer>,
}

impl TestGameView {
    fn setup_level(&mut self) {
        self.level.setup();

        let mut player = self.level.player;
        self.dpad
            .on_press
            .subscribe(move |direction| player.move_by_direction(direction));

        self.left_stick
            .on_direction_change
            .subscribe(move |direction| {
                player.add_impulse(direction);
            });
    }

    fn setup_slider(&mut self) {
        self.scale_slider = add_view_with_frame(self, (50, 280));
        self.scale_slider.set_multiplier(10.0);

        let mut this = self.to_rglica();
        self.scale_slider.on_change.subscribe(move |value| {
            this.level_mut().drawer_mut().set_scale(value);
        });
    }

    fn setup_ui(&mut self) {
        self.set_frame((10, 10, 1000, 500).into());

        self.sprote_view = add_view_with_frame(self, (500, 180));

        let mut this = self.to_rglica();
        self.level
            .base_mut()
            .on_sprite_selected
            .subscribe(move |sprite| this.sprote_view.set_sprite(sprite));
        error!("on_sprite_selected.subscribe: OK");

        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (200, 150).into();

            error!("Frame: OK");

            dpad.set_images(
                Image::get("up.png"),
                Image::get("down.png"),
                Image::get("left.png"),
                Image::get("right.png"),
            );

            error!("Images: OK");
        });
        error!("Dpad: OK");

        self.left_stick = add_view(self);

        self.setup_slider();
        error!("Slider: OK");

        self.test_view = add_view_with_frame(self, (280, 400));
        self.test_view.set_image(Image::get("cat.png"));
        self.test_view.set_button_image(Image::get("square.png"));
        self.test_view.set_animation_image(Image::get("palm.png"));

        self.to_benchmark = add_view(self);
        self.to_benchmark.set_text("Benchmark");
        self.to_benchmark.frame_mut().size = (120, 20).into();
        let mut this = self.to_rglica();
        self.to_benchmark.on_tap.subscribe(move |_| {
            this.ui.set_view(BenchmarkView::boxed());
        });

        error!("to_benchmark: OK");

        let mut this = self.to_rglica();
        self.play = make_view_on(self, |play: &mut Button| {
            play.set_text("Play sound");
            play.frame_mut().size = (120, 20).into();
            play.on_tap.subscribe(move |_| this.sound.play());
        });

        self.sound = Sound::get("retro.wav");
    }
}

impl View for TestGameView {
    fn on_touch(&mut self, touch: &Touch) {
        error!("{}", format!("{:?}", &touch.position));
        let mut view = ViewBase::with_frame((touch.position.x, touch.position.y, 5, 5).into());
        view.set_color(Color::random());
        self.add_subview(view);
    }

    fn setup(&mut self) {
        self.enable_touch();
        self.setup_ui();
        self.setup_level();
    }

    fn layout(&mut self) {
        self.place().as_background();

        self.dpad.place().bottom_left(5);
        self.left_stick
            .place()
            .anchor(self.dpad, Anchor::Right, Anchor::Bot, 20);

        self.scale_slider.place().proportional_height(0.5);
        self.scale_slider
            .place()
            .anchor(self.dpad, Anchor::Top, Anchor::Left, 10);

        self.sprote_view
            .place()
            .anchor(self.scale_slider, Anchor::Right, Anchor::Bot, 10);

        self.test_view.place().bottom_right(20);
        self.test_view.place().proportional_width(0.28);
        self.test_view.place().proportional_height(0.8);

        self.to_benchmark.place().bottom_center(10);

        self.play
            .place()
            .anchor(self.to_benchmark, Anchor::Top, Anchor::Center, 10);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl GameView for TestGameView {
    fn level(&self) -> &dyn Level {
        &self.level
    }
    fn level_mut(&mut self) -> &mut dyn Level {
        &mut self.level
    }

    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
