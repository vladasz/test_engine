use std::f32::consts::PI;

use dispatch::on_main;
use gm::{
    flat::{point_on_circle, Size},
    Color,
};
use refs::Weak;
use rtools::{Animation, Time};
use ui::{
    view, Container, Event, ModalView, UIAnimation, ViewAnimation, ViewCallbacks, ViewData, ViewFrame,
    ViewSetup, ViewSubviews,
};

static CIRCLES_N: u32 = 6;
static mut SPINNER: Weak<Spinner> = Weak::const_default();

#[view]
pub struct Spinner {
    circles: Vec<Weak<Container>>,
    event:   Event<()>,
}

impl Spinner {
    // fn set_alpha(&mut self, alpha: impl IntoF32) {
    //     self.set_color(self.color().with_alpha(alpha));
    //     for cir in &mut self.circles {
    //         let c = *cir.color();
    //         cir.set_color(c.with_alpha(alpha));
    //     }
    // }
}

impl ViewSetup for Spinner {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::GRAY.with_alpha(0.8));
        self.set_corner_radius(20);

        for _ in 0..CIRCLES_N {
            let mut circle = self.internal_add_view::<Container>().weak();

            circle.set_size((16, 16));
            circle.set_color(Color::LIGHT_BLUE);
            circle.set_corner_radius(8);

            self.circles.push(circle);
        }
    }
}

impl ViewCallbacks for Spinner {
    fn update(&mut self) {
        const ONE_SECOND_IN_NANOSECONDS: i64 = 1_000_000_000;
        let current_time: i64 = Time::now();

        let val = ((current_time % ONE_SECOND_IN_NANOSECONDS) as f32) / ONE_SECOND_IN_NANOSECONDS as f32;

        let span = PI * 2.0;
        let start = -PI;

        let angle = start + span * val;

        let step = 2.0 * PI / CIRCLES_N as f32;

        let points: Vec<_> = (0..CIRCLES_N)
            .map(|index| point_on_circle(40.0, angle + step * index as f32, self.size().center()))
            .collect();

        for (view, point) in self.circles.iter_mut().zip(points) {
            view.set_origin((point.x - 8.0, point.y - 8.0));
        }
    }
}

impl Spinner {
    pub fn start() {
        on_main(|| unsafe {
            assert!(SPINNER.is_null(), "A spinner is already spinning");
            SPINNER = Self::prepare_modally(());
        });
    }

    pub fn stop() {
        on_main(|| unsafe {
            assert!(SPINNER.is_ok(), "Spinner already stopped");

            let animation = UIAnimation::new(Animation::new(0.8, 0, 0.4), |sp, val| {
                let color = sp.color();
                sp.set_color(color.with_alpha(val));
                for dot in sp.subviews_mut() {
                    let color = *dot.color();
                    dot.set_color(color.with_alpha(val));
                }
            });

            animation.on_finish.sub(|| {
                SPINNER.hide_modal(());
                SPINNER = Default::default();
            });

            SPINNER.add_animation(animation);
        });
    }

    pub fn instant_stop() {
        unsafe {
            assert!(SPINNER.is_ok(), "Spinner already stopped");
            SPINNER.hide_modal(());
            SPINNER = Default::default();
        }
    }
}

impl ModalView for Spinner {
    fn modal_event(&self) -> &Event<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (140, 140).into()
    }
}
