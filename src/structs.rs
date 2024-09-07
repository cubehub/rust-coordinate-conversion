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

use std::convert::From;
use ::conversions::{ecef_to_lla, lla_to_ecef};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature="rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct LLA {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_m:   f64,
}

impl PartialEq for LLA {
    fn eq(&self, other: &LLA) -> bool {
        (self.alt_m   - other.alt_m).abs()   < 0.000_000_1 &&
        (self.lat_deg - other.lat_deg).abs() < 0.000_000_1 &&
        (self.lon_deg - other.lon_deg).abs() < 0.000_000_1
    }
}

impl From<ECEF> for LLA {
    fn from(ecef: ECEF) -> LLA {
        ecef_to_lla(&ecef)
    }
}

impl<'a> From<&'a ECEF> for LLA {
    fn from(ecef: &ECEF) -> LLA {
        ecef_to_lla(ecef)
    }
}

impl<'a> From<&'a LLA> for LLA {
    fn from(lla: &LLA) -> LLA {
        (*lla).clone()
    }
}


#[cfg_attr(feature="rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct ECEF {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for ECEF {
    fn eq(&self, other: &ECEF) -> bool {
        (self.x - other.x).abs() < 0.000_000_1 &&
        (self.y - other.y).abs() < 0.000_000_1 &&
        (self.z - other.z).abs() < 0.000_000_1
    }
}

impl From<LLA> for ECEF {
    fn from(lla: LLA) -> ECEF {
        lla_to_ecef(&lla)
    }
}

impl<'a> From<&'a LLA> for ECEF {
    fn from(lla: &LLA) -> ECEF {
        lla_to_ecef(lla)
    }
}

impl<'a> From<&'a ECEF> for ECEF {
    fn from(ecef: &ECEF) -> ECEF {
        (*ecef).clone()
    }
}
