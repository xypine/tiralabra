use std::fs;

use aaltofunktionromautus::{
    rules::RuleSet2D,
    tile_extraction::{
        TileExtractor,
        overlapping_bitmap::{OverlappingBitmapExtractor, OverlappingBitmapExtractorOptions},
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

    let rules2: RuleSet2D = serde_json::from_str(include_str!("../rules.json")).unwrap();

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
