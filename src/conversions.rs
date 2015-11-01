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

use std::f64::consts::PI;
use ::structs::{ECEF, LLA};

// Conversion formulas from https://microem.ru/files/2012/08/GPS.G1-X-00006.pdf

// Semi-major axis of the Earth in meters
const WGS84_A: f64 = 6378137.0;
// Inverse flattening of the Earth
const WGS84_IF: f64 = 298.257223563;
// Flattening of the Earth
const WGS84_F: f64 = 1./WGS84_IF;
// Semi-minor axis of the Earth in meters
const WGS84_B: f64 = WGS84_A*(1.-WGS84_F);

lazy_static! {
    // Eccentricity of the Earth
    // static ref WGS84_E:  f64 = (2.*WGS84_F - WGS84_F*WGS84_F).sqrt();
    static ref WGS84_E:  f64 = ((WGS84_A*WGS84_A-WGS84_B*WGS84_B)/(WGS84_A*WGS84_A)).sqrt();
    // Second eccentricity of the Earth
    static ref WGS84_SE: f64 = ((WGS84_A*WGS84_A-WGS84_B*WGS84_B)/(WGS84_B*WGS84_B)).sqrt();
}


pub fn lla_to_ecef(lla: &LLA) -> ECEF {
    let d = *WGS84_E * to_rad(lla.lat_deg).sin();
    let n = WGS84_A / (1. - d*d).sqrt();
    let tmp = (n + lla.alt_m) * to_rad(lla.lat_deg).cos();
    ECEF {
        x: tmp * to_rad(lla.lon_deg).cos(),
        y: tmp * to_rad(lla.lon_deg).sin(),
        z: ((1. - *WGS84_E*(*WGS84_E))*n + lla.alt_m) * to_rad(lla.lat_deg).sin(),
    }
}

pub fn ecef_to_lla(ecef: &ECEF) -> LLA {
    let p = (ecef.x*ecef.x+ecef.y*ecef.y).sqrt();
    let phi = ((ecef.z*WGS84_A)/(p*WGS84_B)).atan();
    let (sin_phi, cos_phi) = phi.sin_cos();

    let lat_rad = ((    ecef.z + (*WGS84_SE)*(*WGS84_SE) * WGS84_B * sin_phi*sin_phi*sin_phi
                   ) / (
                        p      - (*WGS84_E)*(*WGS84_E)   * WGS84_A * cos_phi*cos_phi*cos_phi
                   )).atan();

    let d = *WGS84_E * lat_rad.sin();
    let n = WGS84_A / (1. - d*d).sqrt();

    LLA {
        lat_deg: to_deg(lat_rad),
        lon_deg: to_deg((ecef.y/ecef.x).atan()),
        alt_m:   (p/lat_rad.cos())-n,
    }
}

fn to_rad(deg: f64) -> f64 {
    (deg/180.)*PI
}

fn to_deg(rad: f64) -> f64 {
    (rad/PI)*180.
}
