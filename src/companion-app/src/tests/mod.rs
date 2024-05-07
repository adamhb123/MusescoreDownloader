#[cfg(test)]

use super::*;
#[test]
fn test_svg_to_png() {
    svg_to_png(Path::new(".\\rsc\\ferris.svg"));
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
