use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Anchor, Color, IntView, Label, Sub, TextAlignment, ViewData, ViewSetup, ViewSubviews, UI},
    ui_test::{helpers::check_colors, inject_touches},
    App,
};

#[view]
struct LabelTestView {
    label:          Sub<Label>,
    text_size_view: Sub<IntView>,
}

impl ViewSetup for LabelTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("ßšėčыў");
        self.label.place().back().size(280, 280).center();

        self.text_size_view
            .place()
            .size(50, 100)
            .center_y()
            .anchor(Anchor::Right, self.label, 10);
        self.text_size_view.set_value(32).set_step(5);

        self.text_size_view.on_change(move |size| {
            self.label.set_text_size(size);
        });
    }
}

pub async fn test_label() -> Result<()> {
    let mut view = UI::init_test_view::<LabelTestView>().await;

    App::set_window_size((400, 400));

    check_colors(
        r#"
              36  109 -  25  51  76
             149  173 - 255 255 255
             191  171 - 255 255 255
             229  173 - 255 255 255
             265  176 - 255 255 255
             311  175 - 255 255 255
             267  198 - 255 255 255
             227  196 - 255 255 255
             179  196 - 155 155 155
             147  195 - 255 255 255
             127  216 - 255 255 255
             158  215 - 255 255 255
             197  215 - 255 255 255
             245  213 - 102 102 102
             286  215 - 255 255 255
             150  196 -   1   1   1
             209  199 - 255 255 255
             237  197 - 255 255 255
             263  203 - 255 255 255
             214  279 - 255 255 255
             201  119 - 255 255 255
             192   42 -  25  51  76
             180  371 -  25  51  76
    "#,
    )
    .await?;

    inject_touches(
        r#"
            1   200  b
            31   200  e
            30   200  b
            30   200  e
            30   201  b
            30   201  e
            30   201  b
            30   201  e
            29   201  b
            29   201  e
            29   201  b
            29   201  e
            29   201  b
            29   201  e
            30   201  b
            30   201  e
            30   201  b
            30   201  e
            30   201  b
            30   201  e

    "#,
    )
    .await;

    from_main(move || {
        view.label.set_text_color(Color::BLUE);
    })
    .await;

    check_colors(
        r#"
              87  150 - 255 255 255
              85  181 - 255 255 255
              84  200 - 255 255 255
              93  214 - 180 180 239
              94  239 - 255 255 255
             123  234 - 255 255 255
             129  208 - 255 255 255
             131  183 -   0   0 203
             135  161 - 255 255 255
             158  149 - 255 255 255
             180  156 - 255 255 255
             178  192 - 255 255 255
             175  212 - 255 255 255
             176  237 - 255 255 255
             217  234 - 255 255 255
             230  228 - 255 255 255
             229  215 -   1   1 203
             227  202 - 255 255 255
             220  174 - 255 255 255
             219  141 - 255 255 255
             236  137 - 255 255 255
             264  159 - 255 255 255
             262  184 - 255 255 255
             261  215 - 255 255 255
             261  228 - 255 255 255
             292  225 - 255 255 255
             289  206 -   1   1 203
             298  149 - 255 255 255
             320  146 - 255 255 255
             326  169 -   0   0 203
             318  199 - 255 255 255
             316  216 -   1   1 203
             316  234 -  23  23 207
             336  232 - 255 255 255
             318  229 -   1   1 203
             305  229 - 255 255 255
             303  210 - 255 255 255
             285  205 -  36  36 211
             238  202 - 255 255 255
             204  199 -   0   0 203
             160  196 -   1   1 203
             112  199 - 255 255 255
              82  206 - 255 255 255
    "#,
    )
    .await?;

    from_main(move || {
        view.label.set_text_size(28);
        view.add_view::<Label>()
            .set_text("Left Left")
            .set_alignment(TextAlignment::Left)
            .place()
            .lrt(60)
            .h(60);
        view.add_view::<Label>()
            .set_text("Right")
            .set_alignment(TextAlignment::Right)
            .place()
            .lrb(60)
            .h(60);
    })
    .await;

    check_colors(
        r#"
             221  306 - 255 255 255
             239  305 - 255 255 255
             258  304 - 255 255 255
             267  304 -   0   0   0
             279  304 - 255 255 255
             293  305 -   0   0   0
             314  302 - 255 255 255
             319  302 -   0   0   0
             329  302 - 255 255 255
             348  306 -  25  51  76
             274  282 - 255 255 255
             282  290 - 255 255 255
             293  301 - 255 255 255
             300  315 -   1   1   1
             305  328 - 255 255 255
             312  341 -  25  51  76
             318  348 -  25  51  76
              42   78 -  25  51  76
              54   79 -  25  51  76
              70   83 - 255 255 255
              91   85 - 255 255 255
             126   87 - 255 255 255
             149   88 - 255 255 255
             156   89 - 255 255 255
             179   88 - 152 152 152
             181   88 - 255 255 255
             191   88 - 255 255 255
             214   88 - 255 255 255
             224   89 - 255 255 255
             181   52 -  25  51  76
             176   53 -  25  51  76
             128   73 - 255 255 255
             113   92 - 255 255 255
             111  100 - 255 255 255
             102  114 - 255 255 255
        "#,
    )
    .await?;

    debug!("Label test: OK");

    Ok(())
}
