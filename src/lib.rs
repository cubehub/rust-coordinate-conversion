/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Mattis Marjak (mattis.marjak@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod structs;

pub use structs::{ECEF, LLA};

// Semi-major axis of the Earth in meters
const WGS84_A: f64 = 6378137.0;
// Inverse flattening of the Earth
const WGS84_IF: f64 = 298.257223563;
// Flattening of the Earth
const WGS84_F: f64 = 1./WGS84_IF;
// Semi-minor axis of the Earth in meters
// const WGS84_B: f64 = WGS84_A*(1.-WGS84_F);
// Eccentricity of the Earth
lazy_static! {
    static ref WGS84_E: f64 = (2.*WGS84_F - WGS84_F*WGS84_F).sqrt();
}

pub fn lla_to_ecef(lla: &LLA) -> ECEF {
    let d = *WGS84_E * lla.lat_deg.sin();
    let n = WGS84_A / (1. - d*d).sqrt();
    let tmp = (n + lla.alt_m) * lla.lat_deg.cos();
    ECEF {
        x: tmp * lla.lon_deg.cos(),
        y: tmp * lla.lon_deg.sin(),
        z: ((1. - *WGS84_E**WGS84_E)*n + lla.alt_m) * lla.lat_deg.sin(),
    }
}

#[test]
fn lla_to_ecef_conv() {
    let lla = LLA {
        lat_deg : 0.,
        lon_deg : 0.,
        alt_m   : 0.,
    };
    let ecef = lla_to_ecef(&lla);
    println!("lla:  {:?}", lla);
    println!("ecef: {:?}", ecef);
    unimplemented!();
}
