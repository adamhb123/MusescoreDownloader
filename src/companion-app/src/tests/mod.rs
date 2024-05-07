#[cfg(test)]

use super::*;
#[test]
fn test_svg_to_png() {
    let mut svg = std::env::current_dir()
        .unwrap().canonicalize().unwrap();
    svg.push("src/tests/rsc/score_1.svg");
    svg_to_png(&svg);
}

#[test]
fn test_config() {
    msd_config::generate_default_config().unwrap();
}

//     #[test]
//     #[should_panic(expected = "Divide result is zero")]
//     fn test_specific_panic() {
//         divide_non_zero_result(1, 10);
//     }
