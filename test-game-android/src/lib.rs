#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: test_game::AndroidApp) {
    test_game::start_test_game(app);
}

use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

// Works
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_test_1game_MainActivity_setFilesDir<'local>(
    mut env: JNIEnv<'local>,
    _: JClass,
    input: JString<'local>,
) {
    use test_game::test_engine::Paths;
    let input: String = env.get_string(&input).expect("Couldn't get java string!").into();
    Paths::set_storage_path(input);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_test_1game_MainActivity_setAssetManager<'local>(
    _env: JNIEnv<'local>,
    _: JClass,
    _input: JClass,
) {
    dbg!("Java_com_example_test_1game_MainActivity_setAssetManager");
}
