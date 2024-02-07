#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;

use anyhow::Result;
use test_engine::{git_root, ui::ViewSetup, Assets, TEApp, WGPUApp};

use crate::interface::wgpu_test_view::WGPUTestView;

#[tokio::main]
async fn main() -> Result<()> {
    Assets::init(git_root().expect("git_root()"));
    WGPUApp::start(TEApp::new(WGPUTestView::new()), 1200, 1200).await
}
