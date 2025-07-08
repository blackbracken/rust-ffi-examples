pub mod algorithms;

use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::jint;

// cf. https://docs.rs/jni/latest/jni/
// for Android
#[unsafe(no_mangle)]
pub extern "system" fn Java_black_bracken_rustffiexamples_android_Randomizer_genRandomNumber<'local>(
    _: JNIEnv<'local>,
    _: JClass<'local>,
    seed: jint,
) -> jint {
    let seed = seed as u32;
    genRandomNumber(seed) as jint
}

fn genRandomNumber(seed: u32) -> u32 {
    let mut rng = algorithms::mersenne_twister::MersenneTwister::new(seed);
    rng.next_u32()
}