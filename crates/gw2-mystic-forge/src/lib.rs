//! Guild Wars 2 Mystic Forge recipes as static data.
//!
//! The official API has no Mystic Forge recipes, yet every legendary's final
//! step (and Gift of Mastery, Gift of Fortune, Mystic Clovers, …) is one. This
//! crate bundles them as compiled-in `static` data: no parsing, no allocation,
//! no init at runtime.
//!
//! Both the Rust data (`src/generated.rs`) and its JSON twin
//! (`data/mystic_forge.json`, the reviewable, language-neutral form) are
//! written by the `generate-mystic-forge` binary (feature `generate`) from the
//! GW2 wiki's Semantic MediaWiki data. Neither is edited by hand. Only records
//! that pass the generator's checks are included; everything it dropped is
//! listed with a reason in `data/excluded.json`.
//!
//! ```
//! // Twilight: Dusk + Gift of Twilight + Gift of Mastery + Gift of Fortune.
//! let twilight = gw2_mystic_forge::by_output(30704).next().unwrap();
//! assert_eq!(twilight.ingredients.len(), 4);
//! ```

use serde::Serialize;

#[rustfmt::skip]
mod generated;

/// Version of the data format, shared by the Rust types and the JSON file.
/// Bumped on any breaking change to either.
pub const FORMAT_VERSION: u32 = 1;

/// Where the data came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Source {
    /// The wiki the recipes were queried from.
    pub url: &'static str,
    /// Licence and attribution notice for the wiki data.
    pub license: &'static str,
    /// The game build (`/v2/build`) the item ids were validated against. A
    /// newer build is the cue to regenerate.
    pub game_build: u32,
}

/// One Mystic Forge recipe.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Recipe {
    /// The wiki's Mystic Forge id: output item id × 100 + the recipe's number
    /// on the output's page. Unique and stable across regenerations.
    pub id: u32,
    /// The wiki subobject the recipe came from (`"Twilight#recipe1"`), to find
    /// it again on the wiki.
    pub wiki: &'static str,
    /// Output item id.
    pub output: u32,
    #[serde(rename = "yield")]
    pub yield_: Yield,
    /// One to four stacks, in forge-slot order. Item ids only: the forge
    /// takes no currencies.
    pub ingredients: &'static [Ingredient],
}

/// What a recipe produces. Only [`Yield::Fixed`] is a promise.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Yield {
    /// Always exactly this many.
    Fixed(u32),
    /// A random result: this many of the output is one possible outcome
    /// (e.g. the classic Mystic Clover combine). Not usable for planning.
    Random(u32),
    /// The expected output per craft of a random recipe, as published on the
    /// wiki (Mystic Clover: 0.31). For cost estimates, never for counts. None
    /// are in the current data: the wiki keeps its average recipes in hidden
    /// blocks without ingredient data, so the generator excludes them.
    Average(f64),
}

impl Yield {
    /// Whether a plan can rely on this recipe producing its output.
    pub fn is_fixed(&self) -> bool {
        matches!(self, Yield::Fixed(_))
    }
}

/// One forge slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Ingredient {
    pub item: u32,
    pub count: u32,
}

/// Where the bundled data came from.
pub fn source() -> &'static Source {
    &generated::SOURCE
}

/// Every bundled recipe, sorted by [`Recipe::id`].
pub fn recipes() -> &'static [Recipe] {
    generated::RECIPES
}

/// Recipe by its Mystic Forge id.
pub fn get(id: u32) -> Option<&'static Recipe> {
    let recipes = recipes();
    recipes
        .binary_search_by_key(&id, |r| r.id)
        .ok()
        .map(|i| &recipes[i])
}

/// Every recipe that produces `item`, fixed and random alike.
pub fn by_output(item: u32) -> impl Iterator<Item = &'static Recipe> {
    let index = generated::BY_OUTPUT;
    let slots: &'static [u32] = match index.binary_search_by_key(&item, |&(output, _)| output) {
        Ok(i) => index[i].1,
        Err(_) => &[],
    };
    slots.iter().map(|&i| &generated::RECIPES[i as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_data_is_consistent() {
        assert!(source().game_build > 0);
        let recipes = recipes();
        assert!(!recipes.is_empty());
        assert!(
            recipes.windows(2).all(|w| w[0].id < w[1].id),
            "sorted by id, ids unique"
        );
        for r in recipes {
            assert!(
                (1..=4).contains(&r.ingredients.len()),
                "{}: {} ingredients",
                r.wiki,
                r.ingredients.len()
            );
            assert!(r.ingredients.iter().all(|i| i.count > 0), "{}", r.wiki);
        }
    }

    #[test]
    fn output_index_covers_every_recipe() {
        let indexed: usize = generated::BY_OUTPUT.iter().map(|(_, s)| s.len()).sum();
        assert_eq!(indexed, recipes().len());
        assert!(
            generated::BY_OUTPUT.windows(2).all(|w| w[0].0 < w[1].0),
            "index sorted by output"
        );
        for r in recipes() {
            assert!(by_output(r.output).any(|x| x.id == r.id), "{}", r.wiki);
            assert_eq!(get(r.id).map(|x| x.id), Some(r.id));
        }
    }

    #[test]
    fn twilight_is_a_fixed_forge_recipe() {
        let r = by_output(30704).next().expect("Twilight recipe");
        assert_eq!(r.yield_, Yield::Fixed(1));
        let items: Vec<u32> = r.ingredients.iter().map(|i| i.item).collect();
        // Dusk, Gift of Twilight, Gift of Mastery, Gift of Fortune.
        assert_eq!(items, [29185, 19648, 19674, 19626]);
    }

    #[test]
    fn mystic_clover_is_only_random() {
        let clover: Vec<_> = by_output(19675).collect();
        assert!(!clover.is_empty());
        assert!(clover.iter().all(|r| matches!(r.yield_, Yield::Random(_))));
    }
}
