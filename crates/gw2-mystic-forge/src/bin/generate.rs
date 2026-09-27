//! Regenerate the Mystic Forge data from the GW2 wiki: `src/generated.rs`
//! (the compiled-in `static` data) and `data/mystic_forge.json` (its
//! reviewable, language-neutral twin), plus `data/excluded.json`.
//!
//!     cargo run -p gw2-mystic-forge --features generate --bin generate-mystic-forge
//!
//! The wiki stores every `{{Recipe}}` as a Semantic MediaWiki subobject, so one
//! paged `ask` query returns all Mystic Forge recipes with game ids — no page
//! scraping. Each record is then checked; only records that pass land in the
//! dataset, the rest go to `data/excluded.json` with the reason, for review or
//! a hand patch.
//!
//! The generator compiles against the library, so it needs a `generated.rs`
//! that builds; if a bad one ever lands, `git checkout src/generated.rs` first.
//!
//! Ingredient ids come from the `Has ingredient with id` subobjects. A recipe
//! with an ingredient that has no id is excluded: on the wiki those are
//! wildcards ("any exotic sword", "any Charm"), not one item.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use gw2_api::Gw2Client;
use gw2_mystic_forge::{FORMAT_VERSION, Ingredient, Yield};
use serde::Serialize;
use serde_json::Value;

const WIKI: &str = "https://wiki.guildwars2.com";
const USER_AGENT: &str = concat!(
    "gw2-mystic-forge/",
    env!("CARGO_PKG_VERSION"),
    " (recipe data generator; https://github.com/VeeLume/gw2-companion)"
);
const LICENSE: &str = "Recipe data compiled by the contributors of the Guild Wars 2 Wiki \
(https://wiki.guildwars2.com), wiki content under GFDL 1.3. Game data \u{a9} ArenaNet.";

/// Printouts, with labels so the two ingredient chains cannot collide.
const QUERY: &str = "[[Has context::Recipe]][[Has recipe source::Mystic forge]]\
|?Has output game id=output\
|?Has output quantity=quantity\
|?Has mystic forge id=mfid\
|?Can be queried for base ingredients=queryable\
|?Has ingredient with id.Has ingredient id=id_ids\
\
|?Has ingredient with id.Has ingredient quantity=id_counts\
\
|?Has ingredient.Has ingredient quantity=all_counts\
|limit=500";

/// The raw record, as the wiki returns it.
struct Raw {
    wiki: String,
    output: Vec<f64>,
    quantity: Vec<f64>,
    mfid: Vec<f64>,
    queryable: Option<bool>,
    id_ids: Vec<f64>,
    id_counts: Vec<f64>,
    all_counts: Vec<f64>,
}

/// A checked recipe: the owned twin of `gw2_mystic_forge::Recipe`, whose
/// fields borrow from the compiled-in data.
#[derive(Serialize)]
struct Built {
    id: u32,
    wiki: String,
    output: u32,
    #[serde(rename = "yield")]
    yield_: Yield,
    ingredients: Vec<Ingredient>,
}

#[derive(Serialize)]
struct SourceOut {
    url: &'static str,
    license: &'static str,
    game_build: u32,
}

#[derive(Serialize)]
struct Excluded {
    wiki: String,
    reason: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let data_dir = crate_dir.join("data");
    let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let raws = fetch_all(&http).await?;
    eprintln!("wiki: {} Mystic Forge recipe records", raws.len());

    let api = Gw2Client::new();
    let game_build = api.build().get().await.context("GET /v2/build")?.id;
    let known_items: HashSet<u32> = api
        .items()
        .list_ids()
        .await
        .context("GET /v2/items")?
        .into_iter()
        .map(|id| id.0)
        .collect();
    eprintln!("api: build {game_build}, {} item ids", known_items.len());

    let mut recipes = Vec::new();
    let mut excluded = Vec::new();
    for raw in &raws {
        match convert(raw, &known_items) {
            Ok(recipe) => recipes.push(recipe),
            Err(reason) => excluded.push(Excluded {
                wiki: raw.wiki.clone(),
                reason,
            }),
        }
    }

    // The same recipe can be recorded on several pages (e.g. both halves of a
    // gen-1 legendary): keep the lowest id of each identical recipe.
    recipes.sort_by_key(|r| r.id);
    let mut seen: HashMap<String, u32> = HashMap::new();
    recipes.retain(|r| {
        let mut ingredients: Vec<_> = r.ingredients.iter().map(|i| (i.item, i.count)).collect();
        ingredients.sort_unstable();
        let key = format!("{}|{:?}|{:?}", r.output, r.yield_, ingredients);
        match seen.get(&key) {
            Some(&kept) => {
                excluded.push(Excluded {
                    wiki: r.wiki.clone(),
                    reason: format!("duplicate of recipe {kept}"),
                });
                false
            }
            None => {
                seen.insert(key, r.id);
                true
            }
        }
    });
    excluded.sort_by(|a, b| a.wiki.cmp(&b.wiki));

    let set = RecipeSetOut {
        format: FORMAT_VERSION,
        source: SourceOut {
            url: WIKI,
            license: LICENSE,
            game_build,
        },
        recipes: &recipes,
    };
    write_json(&data_dir.join("mystic_forge.json"), &set)?;
    write_json(&data_dir.join("excluded.json"), &excluded)?;
    write_rust(
        &crate_dir.join("src").join("generated.rs"),
        game_build,
        &recipes,
    )?;

    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
    for e in &excluded {
        let kind = e.reason.split(':').next().unwrap_or(&e.reason);
        let kind = kind.split(" of recipe").next().unwrap_or(kind);
        *reasons.entry(kind.to_string()).or_default() += 1;
    }
    eprintln!(
        "kept {} recipes, excluded {}:",
        set.recipes.len(),
        excluded.len()
    );
    for (reason, n) in reasons {
        eprintln!("  {n:>5}  {reason}");
    }
    Ok(())
}

/// The JSON file's shape.
#[derive(Serialize)]
struct RecipeSetOut<'a> {
    format: u32,
    source: SourceOut,
    recipes: &'a [Built],
}

async fn fetch_all(http: &reqwest::Client) -> Result<Vec<Raw>> {
    let mut out = Vec::new();
    let mut offset = 0u64;
    loop {
        let query = format!("{QUERY}|offset={offset}");
        let body: Value = http
            .get(format!("{WIKI}/api.php"))
            .query(&[("action", "ask"), ("format", "json"), ("query", &query)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        if let Some(err) = body.get("error") {
            bail!("wiki ask query failed: {err}");
        }
        // An empty result set is `[]`, a non-empty one an object.
        if let Some(results) = body["query"]["results"].as_object() {
            for (wiki, rec) in results {
                out.push(parse_raw(wiki, &rec["printouts"]));
            }
        }
        match body.get("query-continue-offset").and_then(Value::as_u64) {
            Some(next) => offset = next,
            None => break,
        }
        // Be a polite client of a community wiki.
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(out)
}

fn parse_raw(wiki: &str, p: &Value) -> Raw {
    let nums = |k: &str| -> Vec<f64> {
        p[k].as_array()
            .map(|a| a.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default()
    };
    // Booleans arrive as `"t"`/`"f"` or as JSON booleans depending on the SMW version.
    let queryable = p["queryable"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|v| {
            v.as_bool().or_else(|| match v.as_str() {
                Some("t" | "true" | "1") => Some(true),
                Some("f" | "false" | "0") => Some(false),
                _ => None,
            })
        });
    Raw {
        wiki: wiki.to_string(),
        output: nums("output"),
        quantity: nums("quantity"),
        mfid: nums("mfid"),
        queryable,
        id_ids: nums("id_ids"),
        id_counts: nums("id_counts"),
        all_counts: nums("all_counts"),
    }
}

/// Check one record: the recipe, or the reason it is excluded.
fn convert(raw: &Raw, known_items: &HashSet<u32>) -> std::result::Result<Built, String> {
    let single = |v: &[f64], what: &str| -> std::result::Result<f64, String> {
        match v {
            [x] => Ok(*x),
            [] => Err(format!("no {what}")),
            _ => Err(format!("several {what}s: {v:?}")),
        }
    };
    let id =
        as_u32(single(&raw.mfid, "mystic forge id")?).ok_or("mystic forge id not an integer")?;
    let output = as_u32(single(&raw.output, "output id")?).ok_or("output id not an integer")?;
    let quantity = single(&raw.quantity, "output quantity")?;
    if quantity <= 0.0 {
        return Err(format!("output quantity {quantity}"));
    }

    // `Can be queried for base ingredients` is how the wiki separates recipes
    // a calculator may use from random ones; a fractional quantity on a
    // queryable recipe is a published average.
    let yield_ = match (raw.queryable, as_u32(quantity)) {
        (Some(true), Some(n)) => Yield::Fixed(n),
        (Some(true), None) => Yield::Average(quantity),
        (Some(false), Some(n)) => Yield::Random(n),
        (Some(false), None) => {
            return Err(format!("random recipe with fractional quantity {quantity}"));
        }
        (None, _) => return Err("no base-ingredients flag".to_string()),
    };

    // `all_counts` has one entry per ingredient, `id_ids` only for those with an id.
    if raw.id_ids.len() < raw.all_counts.len() {
        return Err("ingredient without id (a wildcard such as \"any exotic sword\")".to_string());
    }
    if raw.id_ids.len() != raw.id_counts.len() {
        return Err("ingredient ids and counts do not pair".to_string());
    }
    let pairs: Vec<(f64, f64)> = raw
        .id_ids
        .iter()
        .copied()
        .zip(raw.id_counts.iter().copied())
        .collect();

    if pairs.is_empty() || pairs.len() > 4 {
        return Err(format!("{} ingredients", pairs.len()));
    }
    let mut ingredients = Vec::with_capacity(pairs.len());
    for (item, count) in pairs {
        let item = as_u32(item).ok_or_else(|| format!("ingredient id {item} not an integer"))?;
        let count = as_u32(count)
            .filter(|&c| c > 0)
            .ok_or_else(|| format!("ingredient count {count} for item {item}"))?;
        ingredients.push(Ingredient { item, count });
    }

    for item in std::iter::once(output).chain(ingredients.iter().map(|i| i.item)) {
        if !known_items.contains(&item) {
            return Err(format!("unknown item: {item} not in /v2/items"));
        }
    }

    Ok(Built {
        id,
        wiki: raw.wiki.clone(),
        output,
        yield_,
        ingredients,
    })
}

fn as_u32(x: f64) -> Option<u32> {
    (x.fract() == 0.0 && x >= 0.0 && x <= f64::from(u32::MAX)).then_some(x as u32)
}

fn write_json<T: Serialize + ?Sized>(path: &Path, value: &T) -> Result<()> {
    std::fs::create_dir_all(path.parent().unwrap())?;
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    std::fs::write(path, text).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

/// Write the compiled-in data: the recipes sorted by id, plus an index from
/// output item to recipe positions, sorted by output for binary search. One
/// recipe per line keeps regeneration diffs readable.
fn write_rust(path: &Path, game_build: u32, recipes: &[Built]) -> Result<()> {
    use std::fmt::Write as _;

    let mut out = String::new();
    writeln!(
        out,
        "// @generated by `generate-mystic-forge` from {WIKI}. Do not edit;"
    )?;
    writeln!(out, "// regenerate with:")?;
    writeln!(
        out,
        "//     cargo run -p gw2-mystic-forge --features generate --bin generate-mystic-forge"
    )?;
    writeln!(out, "// {LICENSE}")?;
    writeln!(out)?;
    writeln!(
        out,
        "use crate::{{Ingredient as I, Recipe, Source, Yield}};"
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "pub(crate) static SOURCE: Source = Source {{ url: {WIKI:?}, license: {LICENSE:?}, game_build: {game_build} }};"
    )?;
    writeln!(out)?;
    writeln!(out, "pub(crate) static RECIPES: &[Recipe] = &[")?;
    for r in recipes {
        let ingredients: Vec<String> = r
            .ingredients
            .iter()
            .map(|i| format!("I {{ item: {}, count: {} }}", i.item, i.count))
            .collect();
        writeln!(
            out,
            "    Recipe {{ id: {}, wiki: {:?}, output: {}, yield_: Yield::{:?}, ingredients: &[{}] }},",
            r.id,
            r.wiki,
            r.output,
            r.yield_,
            ingredients.join(", ")
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    let mut by_output: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (i, r) in recipes.iter().enumerate() {
        by_output.entry(r.output).or_default().push(i);
    }
    writeln!(out, "pub(crate) static BY_OUTPUT: &[(u32, &[u32])] = &[")?;
    for (output, slots) in by_output {
        let slots: Vec<String> = slots.iter().map(ToString::to_string).collect();
        writeln!(out, "    ({output}, &[{}]),", slots.join(", "))?;
    }
    writeln!(out, "];")?;

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}
