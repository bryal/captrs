extern crate captrs2;
extern crate qoi;

use std::{fs::File, io::Write, path::Path};

use captrs2::Capturer;
use qoi::encode_to_vec;

fn main() {
    let mut capturer = Capturer::new(1).unwrap();

    let (w, h) = capturer.geometry();

    let f1 = capturer.capture_frame_components().unwrap();
    let f2 = capturer.capture_frame_components().unwrap();

    let p1 = Path::new("./frame1.qoi");
    let p2 = Path::new("./frame2.qoi");

    let o1 = encode_to_vec(&f1, w, h).unwrap();
    let o2 = encode_to_vec(&f2, w, h).unwrap();

    File::create(p1).unwrap().write_all(&o1).unwrap();
    File::create(p2).unwrap().write_all(&o2).unwrap();
}
