use std::fs;

use aaltofunktionromautus::{
    rules::RuleSet2D,
    tile_extraction::{
        TileExtractor,
        overlapping::{OverlappingBitmapExtractor, OverlappingBitmapExtractorOptions},
    },
};

#[cfg(not(tarpaulin_include))] // the utility binary doesn't need to be unit tested
pub fn main() {
    let img = image::open("./samples/Flowers.png").expect("failed to open sample image");
    let extractor = OverlappingBitmapExtractor::new(
        img,
        OverlappingBitmapExtractorOptions {
            n: 3,
            periodic_input: true,
            symmetry: 2,
        },
    );
    // println!("{:#?}", ruleset);
    println!(
        "possible tiles: {}",
        extractor.get_rules().possible.iter().count()
    );
    // println!(
    //     "derived rules: {}",
    //     extractor.get_rules().allowed.iter().count()
    // );
    let json = serde_json::to_string(extractor.get_rules()).expect("serializing ruleset to json");
    fs::write("rules.json", json).expect("writing rules.json");

    let mut tile_file = "".to_owned();
    let rules = extractor.get_rules();
    for tile in &rules.possible {
        let repr = rules.visualize_tile(*tile).unwrap();
        tile_file = format!("{tile_file}{repr}\n\n");
    }

    fs::write("tiles.txt", tile_file).expect("writing tiles.txt");

    let rules: RuleSet2D =
        serde_json::from_str(fs::read_to_string("./rules.json").unwrap().as_str()).unwrap();
}
