use super::*;

lazy_static! {
    static ref TEST_LLA: LLA = LLA {
        lat_deg : 59.4372,
        lon_deg : 24.7453,
        alt_m   : 10.,
    };
    static ref TEST_ECEF: ECEF = ECEF {
        x : 2952716.45,
        y : 1360927.25,
        z : 5468869.50,
    };
}


#[test]
fn ecef_to_lla_precision() {
    let lla2 = ecef_to_lla(&*TEST_ECEF);
    println!("");
    println!("lla:  {:?}", *TEST_LLA);
    println!("lla2: {:?}", lla2);
    assert_eq!(*TEST_LLA, lla2);
}

#[test]
fn lla_to_ecef_precision() {
    let ecef2 = lla_to_ecef(&*TEST_LLA);
    println!("");
    println!("ecef:  {:?}", *TEST_ECEF);
    println!("ecef2: {:?}", ecef2);
    assert_eq!(*TEST_ECEF, ecef2);
}

#[test]
fn lla_ecef_lla() {
    println!("");
    println!("lla:   {:?}", *TEST_LLA);
    println!("ecef:  {:?}", *TEST_ECEF);
    let ecef2 = lla_to_ecef(&*TEST_LLA);
    let lla2  = ecef_to_lla(&ecef2);
    println!("ecef2: {:?}", ecef2);
    println!("lla2:  {:?}", lla2);
    assert_eq!(*TEST_LLA, lla2);
}

#[test]
fn ecef_lla_ecef() {
    println!("");
    println!("ecef:  {:?}", *TEST_ECEF);
    println!("lla:   {:?}", *TEST_LLA);
    let lla2  = ecef_to_lla(&*TEST_ECEF);
    let ecef2 = lla_to_ecef(&lla2);
    println!("lla2:  {:?}", lla2);
    println!("ecef2: {:?}", ecef2);
    assert_eq!(*TEST_ECEF, ecef2);
}
