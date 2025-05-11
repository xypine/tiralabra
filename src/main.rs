//! Utility program that extracts rules from sample images
//! lib.rs is the correct entrypoint for the library

use std::fs;

use aaltofunktionromautus::{
    rules::RuleSet2D,
    tile_extraction::{
        TileExtractor,
        overlapping_bitmap::{OverlappingBitmapExtractor, OverlappingBitmapExtractorOptions},
    },
};

#[cfg(not(tarpaulin_include))] // the utility binary doesn't need to be unit tested
fn extract_rules_to_json(
    input_path: &str,
    output_path: &str,
    options: OverlappingBitmapExtractorOptions,
) {
    let img = image::open(input_path).expect("failed to open image");
    let extractor = OverlappingBitmapExtractor::new(img, options);

    println!(
        "possible tiles in \"{input_path}\": {}",
        extractor.get_rules().possible.iter().count()
    );

    let json = serde_json::to_string(extractor.get_rules()).expect("serializing ruleset to json");
    fs::write(output_path, json).expect("writing rules.json");
}

#[cfg(not(tarpaulin_include))] // the utility binary doesn't need to be unit tested
pub fn main() {
    extract_rules_to_json(
        "./samples/MoreFlowers.png",
        "./samples/rules/flowers.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 2,
        },
    );
    extract_rules_to_json(
        "./samples/Link.png",
        "./samples/rules/link.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 1,
        },
    );
    extract_rules_to_json(
        "./samples/Village.png",
        "./samples/rules/village.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 2,
        },
    );
    extract_rules_to_json(
        "./samples/SimpleWall.png",
        "./samples/rules/simple_wall.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 2,
        },
    );
    extract_rules_to_json(
        "./samples/Skyline2.png",
        "./samples/rules/skyline2.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 2,
        },
    );
    extract_rules_to_json(
        "./samples/edge.png",
        "./samples/rules/edge.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: false,
            symmetry: 1,
        },
    );
    extract_rules_to_json(
        "./samples/Water.png",
        "./samples/rules/water.json",
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: false,
            symmetry: 1,
        },
    );

    // let mut tile_file = "".to_owned();
    // let rules = extractor.get_rules();
    // for tile in &rules.possible {
    //     let repr = rules.visualize_tile(*tile).unwrap();
    //     tile_file = format!("{tile_file}{repr}\n\n");
    // }
    //
    // fs::write("tiles.txt", tile_file).expect("writing tiles.txt");
    //
    // let rules: RuleSet2D =
    //     serde_json::from_str(fs::read_to_string("./rules.json").unwrap().as_str()).unwrap();
    //
    // let rules2: RuleSet2D = serde_json::from_str(include_str!("../rules.json")).unwrap();

    // let source = "Let's go fishing tomorrow, I know a good spot the rascals haven't found yet. At least I hope so...".to_owned();
    //
    // let options = OverlappingTextExtractorOptions { n: 3 };
    //
    // let extractor = OverlappingTextExtractor::new(source, options);
    //
    // let ruleset = extractor.get_rules();
    // println!(
    //     "possible tokens: {}",
    //     extractor.get_rules().possible.iter().count()
    // );
    //
    // let json = serde_json::to_string(extractor.get_rules()).expect("serializing ruleset to json");
    // fs::write("rules_txt.json", json).expect("writing rules_txt.json");
}
