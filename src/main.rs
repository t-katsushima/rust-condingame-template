#![allow(non_snake_case)]
mod common;
mod entity;
mod extvec;
mod mainfuncs;
mod reader;

#[allow(unused_imports)]
use rand::prelude::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

use common::*;
use entity::*;
use reader::*;

fn main() {
    let mut _rng = thread_rng();

    loop {
        let _ = Hoge::Fuga;

        // input sample
        let n = readUsize();

        // 最初の入力が終わったタイミングで付けないと、前ターンの終了待ち時間まで含まれてしまう
        let system_time = SystemTime::now();

        eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
    }
}
