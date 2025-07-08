pub mod algorithms;

use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::jint;

use wasm_bindgen::prelude::*;

// cf. https://docs.rs/jni/latest/jni/
// for Android
#[unsafe(no_mangle)]
pub extern "system" fn Java_black_bracken_rustffiexamples_android_Randomizer_genRandomNumber<'local>(
    _: JNIEnv<'local>,
    _: JClass<'local>,
    seed: jint,
) -> jint {
    let seed = if seed < 0 { 0 } else { seed as u32 };
    gen_random_number(seed) as jint
}

// for wasm annotation
#[wasm_bindgen]
pub fn genRandomNumber(seed: u32) -> u32 {
    let random_value = gen_random_number(seed);
    random_value 
}

fn gen_random_number(seed: u32) -> u32 {
    let mut rng = algorithms::mersenne_twister::MersenneTwister::new(seed);
    // 32-bit unsigned integerに収まるようマスク
    rng.next_u32() & 0x7FFFFFFF
}