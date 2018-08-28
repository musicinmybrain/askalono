// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License").
// You may not use this file except in compliance with the License.
// A copy of the License is located at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing
// permissions and limitations under the License.

#![doc(hidden)]
/// This module is experimental. Its API should not be considered stable
/// in any form.
use std::borrow::Cow;

use failure::Error;

use license::LicenseType;
use license::TextData;
use store::Store;

#[derive(Serialize, Debug)]
pub struct IdentifiedLicense {
    pub name: String,
    pub kind: LicenseType,
}

#[derive(Serialize, Debug)]
pub struct ScanResult {
    pub score: f32,
    pub license: Option<IdentifiedLicense>,
    pub containing: Vec<ContainedResult>,
}

#[derive(Serialize, Debug)]
pub struct ContainedResult {
    pub score: f32,
    pub license: IdentifiedLicense,
    pub line_range: (usize, usize),
}

/// A `ScanStrategy` can be used as a high-level wrapped over a `Store`'s
/// analysis logic.
///
/// A strategy configured here can be run repeatedly to scan a document for
/// multiple licenses, or to automatically optimize to locate texts within a
/// larger text.
///
/// # Examples
///
/// ```rust,should_panic
/// # use std::error::Error;
/// use askalono::{ScanStrategy, Store};
///
/// # fn main() -> Result<(), Box<Error>> {
/// let store = Store::new();
/// // [...]
/// let strategy = ScanStrategy::new(&store)
///     .confidence_threshold(0.9)
///     .optimize(true);
/// let results = strategy.scan(&"my text to scan".into())?;
/// # Ok(())
/// # }
/// ```
pub struct ScanStrategy<'a> {
    store: &'a Store,
    confidence_threshold: f32,
    shallow_limit: f32,
    optimize: bool,
    max_passes: u16,
}

impl<'a> ScanStrategy<'a> {
    pub fn new(store: &'a Store) -> ScanStrategy<'a> {
        Self {
            store,
            confidence_threshold: 0.8,
            shallow_limit: 0.99,
            optimize: false,
            max_passes: 10,
        }
    }

    pub fn confidence_threshold(mut self, confidence_threshold: f32) -> Self {
        self.confidence_threshold = confidence_threshold;
        self
    }

    pub fn shallow_limit(mut self, shallow_limit: f32) -> Self {
        self.shallow_limit = shallow_limit;
        self
    }

    pub fn optimize(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    pub fn max_passes(mut self, max_passes: u16) -> Self {
        self.max_passes = max_passes;
        self
    }

    pub fn scan(&self, text: &TextData) -> Result<ScanResult, Error> {
        let mut analysis = self.store.analyze(text)?;
        let score = analysis.score;
        let mut license = None;
        let mut containing = Vec::new();

        // meets confidence threshold? record that
        if analysis.score > self.confidence_threshold {
            license = Some(IdentifiedLicense {
                name: analysis.name.clone(),
                kind: analysis.license_type,
            });

            // above the shallow limit -> exit
            if analysis.score > self.shallow_limit {
                return Ok(ScanResult {
                    score,
                    license,
                    containing,
                });
            }
        }

        if self.optimize {
            // repeatedly try to dig deeper
            // this loop effectively iterates once for each license it finds
            let mut current_text: Cow<TextData> = Cow::Borrowed(text);
            for _n in 0..self.max_passes {
                let (optimized, optimized_score) = current_text.optimize_bounds(analysis.data);

                // stop if we didn't find anything acceptable
                if optimized_score < self.confidence_threshold {
                    break;
                }

                // otherwise, save it
                containing.push(ContainedResult {
                    score: optimized_score,
                    license: IdentifiedLicense {
                        name: analysis.name,
                        kind: analysis.license_type,
                    },
                    line_range: optimized.lines_view(),
                });

                // and white-out + reanalyze for next iteration
                current_text = Cow::Owned(optimized.white_out().expect("optimized must have text"));
                analysis = self.store.analyze(&current_text)?;
            }
        }

        Ok(ScanResult {
            score,
            license,
            containing,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct() {
        let store = Store::new();
        ScanStrategy::new(&store);
        ScanStrategy::new(&store).confidence_threshold(0.5);
        ScanStrategy::new(&store)
            .shallow_limit(0.99)
            .optimize(true)
            .max_passes(100);
    }

    #[test]
    fn shallow_scan() {
        let store = create_dummy_store();
        let test_data = TextData::new("lorem ipsum\naaaaa bbbbb\nccccc\nhello");

        // the above text should have a result with a confidence minimum of 0.5
        let strategy = ScanStrategy::new(&store)
            .confidence_threshold(0.5)
            .shallow_limit(0.0);
        let result = strategy.scan(&test_data).unwrap();
        assert!(
            result.score > 0.5,
            format!("score must meet threshold; was {}", result.score)
        );
        assert_eq!(
            result.license.expect("result has a license").name,
            "license-1"
        );

        // but it won't pass with a threshold of 0.8
        let strategy = ScanStrategy::new(&store)
            .confidence_threshold(0.8)
            .shallow_limit(0.0);
        let result = strategy.scan(&test_data).unwrap();
        assert!(result.license.is_none(), "result license is None");
    }

    #[test]
    fn single_optimize() {
        let store = create_dummy_store();
        // this TextData matches license-2 with an overall score of ~0.46 and optimized
        // score of ~0.57
        let test_data =
            TextData::new("lorem\nipsum abc def ghi jkl\n1234 5678 1234\n0000\n1010101010\n\n8888 9999\nwhatsit hello\narst neio qwfp colemak is the best keyboard layout");

        // check that we can spot the gibberish license in the sea of other gibberish
        let strategy = ScanStrategy::new(&store)
            .confidence_threshold(0.5)
            .optimize(true)
            .shallow_limit(1.0);
        let result = strategy.scan(&test_data).unwrap();
        assert!(result.license.is_none(), "result license is None");
        assert_eq!(result.containing.len(), 1);
        let contained = &result.containing[0];
        assert_eq!(contained.license.name, "license-2");
        assert!(
            contained.score > 0.5,
            "contained score is greater than threshold"
        );
    }

    #[test]
    fn find_multiple_licenses() {
        let store = create_dummy_store();
        // this TextData matches license-2 with an overall score of ~0.46 and optimized
        // score of ~0.57
        let test_data =
            TextData::new("lorem\nipsum abc def ghi jkl\n1234 5678 1234\n0000\n1010101010\n\n8888 9999\nwhatsit hello\narst neio qwfp colemak is the best keyboard layout\naaaaa\nbbbbb\nccccc");

        // check that we can spot the gibberish license in the sea of other gibberish
        let strategy = ScanStrategy::new(&store)
            .confidence_threshold(0.5)
            .optimize(true)
            .shallow_limit(1.0);
        let result = strategy.scan(&test_data).unwrap();
        assert!(result.license.is_none(), "result license is None");
        assert_eq!(result.containing.len(), 2);

        // inspect the array and ensure we got both licenses
        let mut found1 = 0;
        let mut found2 = 0;
        for (_, ref contained) in result.containing.iter().enumerate() {
            match contained.license.name.as_ref() {
                "license-1" => {
                    assert!(contained.score > 0.5, "license-1 score meets threshold");
                    found1 += 1;
                }
                "license-2" => {
                    assert!(contained.score > 0.5, "license-2 score meets threshold");
                    found2 += 1;
                }
                _ => {
                    panic!("somehow got an unknown license name");
                }
            }
        }

        assert!(
            found1 == 1 && found2 == 1,
            "found both licenses exactly once"
        );
    }

    fn create_dummy_store() -> Store {
        let mut store = Store::new();
        store.add_license("license-1".into(), "aaaaa\nbbbbb\nccccc".into());
        store.add_license(
            "license-2".into(),
            "1234 5678 1234\n0000\n1010101010\n\n8888 9999".into(),
        );
        store
    }
}
