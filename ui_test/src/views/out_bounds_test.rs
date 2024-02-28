use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{
        view, AnalogStickView, Anchor, ImageView, IntView, Label, SubView, ViewData, ViewFrame, ViewSetup,
        ViewSubviews,
    },
    App,
};

use crate::{view_tests::inject_touches, views::image_view::check_colors};

#[view]
struct OutBoundsView {
    test: SubView<Label>,
    x:    SubView<IntView>,
    y:    SubView<IntView>,
}

impl ViewSetup for OutBoundsView {
    fn setup(mut self: Weak<Self>) {
        self.test.set_text("AA").set_text_size(100).set_frame((200, 200, 200, 200));

        let mut image = self.test.add_view::<ImageView>();
        image.set_image("cat.png");
        image.place().left_half();

        self.test.add_view::<AnalogStickView>();

        self.x.set_step(50);
        self.x
            .on_change(move |val| {
                self.test.set_x(200.0 + val);
            })
            .place()
            .size(60, 200)
            .center();

        self.y.set_step(50);
        self.y
            .on_change(move |val| {
                self.test.set_y(200.0 + val);
            })
            .place()
            .size(60, 200)
            .center_y()
            .anchor(Anchor::Left, self.x, 10);
    }
}

pub async fn test_out_bounds() -> Result<()> {
    App::init_test_view::<OutBoundsView>(600, 600).await;

    inject_touches(
        r#"
            372  307  b
            371  307  e
            372  304  b
            372  304  e
            373  303  b
            373  303  e
            373  303  b
            373  303  e
            373  303  b
            373  303  e
            373  304  b
            373  304  e
        "#,
    )
    .await;

    check_colors(
        r#"
             168  577 -  25  51  76
             192  575 -  25  51  76
             234  576 -  20   7   3
             284  576 -  59  19  10
             352  575 - 255 255 255
             406  577 -  25  51  76
             428  565 -  25  51  76
             422  539 -  25  51  76
             343  537 - 255 255 255
             291  532 - 162  88  87
             234  535 - 175  99  87
             182  536 -  25  51  76
             153  530 -  25  51  76
             168  489 -  25  51  76
             217  459 -  25  51  76
             250  478 -  25  51  76
             264  515 - 184 114 116
             344  527 - 255 255 255
             384  534 - 255 255 255
        "#,
    )
    .await?;

    inject_touches(
        r#"
            308  305  b
            308  305  e
            308  304  b
            308  304  e
            307  304  b
            307  304  e
            307  304  b
            307  304  e
            307  304  b
            307  304  e
            307  304  b
            307  304  e
            307  304  b
            307  304  e
        "#,
    )
    .await;

    check_colors(
        r#"
             504  560 -  25  51  76
             520  563 -  25  51  76
             554  567 - 201 131 132
             583  560 - 186 135 112
             589  549 - 160 103  81
             585  508 - 211 139 140
             579  472 -  25  51  76
             559  469 -  25  51  76
             524  499 -  25  51  76
             539  543 -  25  51  76
             574  545 - 195 126 126
        "#,
    )
    .await?;

    inject_touches(
        r#"
            365  376  b
            365  376  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            365  377  b
            365  377  e
            299  376  b
            299  376  e
            299  375  b
            300  375  e
        "#,
    )
    .await;

    check_colors(
        r#"
             413  102 -  25  51  76
             429   95 -  25  51  76
             456   65 - 190  88  93
             461   56 - 199 123 119
             468   46 - 213 152 137
             491   33 - 157 102  75
             509   27 - 144  89  63
             530   27 - 112  72  51
             550   22 - 147  75  77
             556   15 - 255 255 255
             563   13 -   0   0   0
             566   12 -   0   0   0
             565   35 - 255 255 255
             567   61 - 255 255 255
             566   88 - 255 255 255
             560  123 -  25  51  76
             529  117 -  25  51  76
             504   75 - 155 101  74
             499   55 - 160 110  81
             476   42 - 188 137 113
             443   30 -  25  51  76
             410   31 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        r#"
            298  380  b
            298  380  e
            297  380  b
            297  380  e
            297  380  b
            297  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
            298  380  b
            298  380  e
        "#,
    )
    .await;

    check_colors(
        r#"
             192   42 -  25  51  76
             189   42 -  25  51  76
             142   42 - 255 255 255
              87   43 - 255 255 255
              46   43 -  97  64  45
              38   44 -  95  59  41
              17   62 - 132  88  64
               7   84 - 152 103  75
              21  118 -  25  51  76
              43  140 -  25  51  76
              91  139 -  25  51  76
             122  125 -  25  51  76
              67   10 -   0   0   0
        "#,
    )
    .await?;

    inject_touches(
        r#"
            303  377  b
            303  377  e
            294  302  b
            294  302  e
            374  293  b
            374  293  e
            373  293  b
            372  293  e
            371  293  b
            372  293  e
            373  293  b
            373  293  e
            373  293  b
            373  293  e
            373  293  b
            373  293  e
        "#,
    )
    .await;

    check_colors(
        r#"
             183  244 -  25  51  76
             163  237 -  25  51  76
              81  237 - 255 255 255
              50  241 - 164  92  92
              31  251 - 167  87  87
              18  286 - 117  76  51
              40  331 - 102  68  47
              76  345 - 255 255 255
             115  356 - 255 255 255
             168  365 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        r#"
            372  364  b
            372  364  e
            381  275  b
            381  275  e
            378  293  b
            378  293  e
            378  293  b
            378  293  e
            378  293  b
            378  293  e
            378  293  b
            378  293  e
            378  292  b
            378  292  e
            378  292  b
            378  292  e
        "#,
    )
    .await;

    check_colors(
        r#"
              35  474 -  25  51  76
              27  490 -  25  51  76
              21  526 - 186 110 110
              20  549 - 169  93  92
              33  568 -  89  42  27
              78  579 -   0   0   0
              83  577 - 255 255 255
             110  575 - 255 255 255
             125  572 - 255 255 255
             164  562 -  25  51  76
             188  546 -  25  51  76
             186  494 -  25  51  76
             124  473 -  25  51  76
              98  517 - 255 255 255
              94  539 - 255 255 255
              84  566 -   1   1   1
        "#,
    )
    .await?;

    debug!("Out bounds test: OK");

    Ok(())
}
