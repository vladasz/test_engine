#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: test_game::AndroidApp) {
    test_game::start_test_game(app);
}

use jni::{
    JNIEnv,
    objects::{JClass, JString},
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_example_test_1game_MainActivity_setFilesDir(
    mut env: JNIEnv,
    _: JClass,
    input: JString,
) {
    use test_game::test_engine::store::Paths;
    let input: String = env.get_string(&input).expect("Couldn't get java string!").into();
    Paths::set_storage_path(input);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_example_test_1game_MainActivity_setAssetManager(
    _env: JNIEnv,
    _: JClass,
    _input: JClass,
) {
    dbg!("Java_com_example_test_1game_MainActivity_setAssetManager");
}
