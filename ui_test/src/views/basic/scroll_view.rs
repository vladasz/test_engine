use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Color, ScrollView, Sub, ViewData, ViewSetup},
    ui_test::{
        helpers::{add_corners, check_colors},
        inject_scroll, inject_touches,
    },
    App,
};

#[view]
struct ScrollViewTest {
    scroll: Sub<ScrollView>,
}

impl ViewSetup for ScrollViewTest {
    fn setup(mut self: Weak<Self>) {
        self.scroll.content_size = (600, 600).into();
        self.scroll.place().back();
        add_corners(self.scroll, Color::TURQUOISE);
    }
}

pub async fn test_scroll_view() -> Result<()> {
    let mut view = App::init_test_view::<ScrollViewTest>().await;

    check_colors(
        r#"
              53  554 -   0 255 255
             168  556 -  25  51  76
             340  553 -  25  51  76
             480  553 -  25  51  76
             532  547 -   0 255 255
             561  469 -  25  51  76
             575  372 -  25  51  76
             566  220 -  25  51  76
             561  134 -  25  51  76
             561   60 -   0 255 255
             440   35 -  25  51  76
             293   41 -  25  51  76
             164   45 -  25  51  76
              93   46 -   0 255 255
              60   71 -   0 255 255
             135  403 -  25  51  76
             382  305 -  25  51  76
        "#,
    )
    .await?;

    assert_eq!(view.scroll.content_offset(), 0.0);

    inject_scroll(-5).await;
    assert_eq!(view.scroll.content_offset(), -0.0);

    inject_scroll(-20).await;
    assert_eq!(view.scroll.content_offset(), -0.0);

    inject_scroll(-30).await;
    assert_eq!(view.scroll.content_offset(), -0.0);

    check_colors(
        r#"
              53  554 -   0 255 255
             168  556 -  25  51  76
             340  553 -  25  51  76
             480  553 -  25  51  76
             532  547 -   0 255 255
             561  469 -  25  51  76
             575  372 -  25  51  76
             566  220 -  25  51  76
             561  134 -  25  51  76
             561   60 -   0 255 255
             440   35 -  25  51  76
             293   41 -  25  51  76
             164   45 -  25  51  76
              93   46 -   0 255 255
              60   71 -   0 255 255
             135  403 -  25  51  76
             382  305 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.scroll.content_size = (400, 400).into();
    })
    .await;

    check_colors(
        r#"
              76  420 -  25  51  76
              65  379 -   0 255 255
              67  361 -   0 255 255
             114  346 -  25  51  76
             125  331 -  25  51  76
              79  305 -   0 255 255
              68  300 -   0 255 255
              67  197 -  25  51  76
              67  139 -  25  51  76
              65   97 -   0 255 255
              83   64 -   0 255 255
             144   58 -  25  51  76
             263   58 -  25  51  76
             282   58 -  25  51  76
             340   56 -   0 255 255
             415   61 -  25  51  76
             415   79 -  25  51  76
             335  108 -  25  51  76
             338  126 -  25  51  76
             380  290 -  25  51  76
             345  328 -   0 255 255
             334  354 -   0 255 255
             300  358 -   0 255 255
             284  374 -  25  51  76
             321  423 -  25  51  76
             378  440 -  25  51  76
             376  372 -   0 255 255
             442  353 -  25  51  76
             474  355 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.scroll.content_size = (600, 800).into();
    })
    .await;

    check_colors(
        r#"
             552  132 -  25  51  76
             555  126 -  25  51  76
             553  111 -  25  51  76
             550   87 -   0 255 255
             544   69 -   0 255 255
             528   55 -   0 255 255
             499   49 -  25  51  76
             460   46 -  25  51  76
             401   46 -  25  51  76
             353   45 -  25  51  76
             250   42 -  25  51  76
             178   40 -  25  51  76
             153   41 -  25  51  76
              72   38 -   0 255 255
              65   41 -   0 255 255
              54   65 -   0 255 255
              52  124 -  25  51  76
              52  137 -  25  51  76
        "#,
    )
    .await?;

    inject_scroll(-150).await;
    assert_eq!(view.scroll.content_offset(), -150.0);

    check_colors(
        r#"
             542  514 -  25  51  76
             535  544 -  25  51  76
             534  555 -   0 255 255
             514  562 -   0 255 255
             451  563 -  25  51  76
             361  565 -  25  51  76
             230  562 -  25  51  76
             136  562 -  25  51  76
             100  567 -  25  51  76
              58  567 -   0 255 255
              51  558 -   0 255 255
              52  543 -  25  51  76
              51  530 -  25  51  76
        "#,
    )
    .await?;

    inject_scroll(-1500).await;
    assert_eq!(view.scroll.content_offset(), -200.0);

    check_colors(
        r#"
             532  478 -  25  51  76
             531  498 -  25  51  76
             529  517 -   0 255 255
             520  533 -   0 255 255
             505  550 -   0 255 255
             489  557 -  25  51  76
             318  554 -  25  51  76
             241  552 -  25  51  76
             172  550 -  25  51  76
             100  547 -  25  51  76
              57  543 -   0 255 255
              55  523 -   0 255 255
              51  505 -   0 255 255
              50  484 -  25  51  76
              51  438 -  25  51  76
              51  416 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        "
            556  565  m
            559  565  m
            582  572  m
            590  576  m
            586  578  m
            584  579  m
            578  577  m
            573  574  m
            561  568  m
            551  554  m
            540  540  m
            558  553  m
            572  566  m
            574  567  m
            574  567  b
            577  540  m
            584  455  m
            586  340  m
            587  207  m
            580  52   m
            577  14   m
            576  0    m
            565  146  m
            552  299  m
            543  450  m
            538  577  m
            539  658  m
            560  593  m
            577  388  m
            596  158  m
            597  38   m
            595  -9   m
            592  4    m
            590  266  m
            587  509  m
            584  636  m
            585  659  m
            594  570  m
            606  307  m
            603  135  m
            599  45   m
            595  3    m
            594  -8   m
            592  -22  m
            589  -21  e
            581  4    m
            538  81   m
            519  100  m
            516  111  m
        ",
    )
    .await;

    check_colors(
        r#"
             535  139 -  25  51  76
             535  121 -  25  51  76
             520   91 -   0 255 255
             518   70 -   0 255 255
             454   42 -  25  51  76
             362   30 -  25  51  76
             212   27 -  25  51  76
             132   33 -  25  51  76
             102   42 -  25  51  76
              66   63 -   0 255 255
              56   93 -   0 255 255
              53  111 -  25  51  76
              53  117 -  25  51  76
        "#,
    )
    .await?;

    // from_main(move || {
    //     view.scroll.content_offset = -400.0;
    // })
    // .await;
    //
    // record_ui_test().await;

    debug!("Scroll view test: OK");

    Ok(())
}
