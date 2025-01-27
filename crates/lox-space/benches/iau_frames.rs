/*
 * Copyright (c) 2023. Helge Eichhorn and the LOX contributors
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, you can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lox_bodies::{Jupiter, RotationalElements};
use lox_orbits::{frames::BodyFixed, rotations::Rotation};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn right_ascension() {
    Jupiter.right_ascension(divan::black_box(0.0));
}

#[divan::bench]
fn right_ascension_dot() {
    Jupiter.right_ascension_rate(divan::black_box(0.0));
}

#[divan::bench]
fn declination() {
    Jupiter.declination(divan::black_box(0.0));
}

#[divan::bench]
fn declination_dot() {
    Jupiter.declination_rate(divan::black_box(0.0));
}

#[divan::bench]
fn prime_meridian() {
    Jupiter.rotation_angle(divan::black_box(0.0));
}

#[divan::bench]
fn prime_meridian_dot() {
    Jupiter.rotation_rate(divan::black_box(0.0));
}

#[divan::bench]
fn rotational_elements() {
    Jupiter.rotational_elements(divan::black_box(0.0));
}

#[divan::bench]
fn rotational_element_rates() {
    Jupiter.rotational_element_rates(divan::black_box(0.0));
}

#[divan::bench]
fn rotation() -> Rotation {
    BodyFixed(Jupiter).rotation(divan::black_box(0.0))
}
