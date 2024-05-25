
include build/common.mk

ui:
	UI_TEST_CYCLES=2 cargo run -p ui-test && UI_TEST_CYCLES=4 cargo run -p ui-test --release

ui3:
	cargo run -p ui-benchmark --profile=r3

fix:
	cargo fix --allow-dirty --allow-staged --all

bench:
	cargo run -p ui-benchmark --release

mobile:
	cargo install test-mobile
	test-mobile --path=../test-mobile/mobile-template

ios-lib:
	cargo lipo -p test-game --release

ios-debug:
	cargo lipo -p test-game
	rm -f ./target/universal/release/libtest_game.a
	cp ./target/universal/debug/libtest_game.a ./target/universal/release/libtest_game.a

.PHONY: mobile
