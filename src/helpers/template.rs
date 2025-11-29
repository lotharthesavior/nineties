use std::collections::HashMap;
use std::fs;
use std::io::Error;
use once_cell::sync::Lazy;
use tera::{Context, Tera};

/// Cached Tera instance - compiled once at startup
static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    match Tera::new("src/resources/views/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
});

/// Cached manifest assets - parsed once at startup
static MANIFEST_ASSETS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    parse_manifest_assets()
});

pub fn load_template(template: &str, params: Vec<(&str, &str)>, assets: Option<Vec<&str>>) -> String {
    let mut context: Context = Context::new();
    for (key, value) in params.into_iter() {
        context.insert(key, value);
    }

    if !context.contains_key("session_message") {
        context.insert("session_message", "");
    }

    context.insert("assets", &get_assets_string(assets));

    TEMPLATES.render(template, &context).expect("Failed to render template")
}

/// Returns the HTML string to add the assets to the template.
/// If the assets are passed, we only add the assets passed, otherwise we add all the assets from
/// the manifest.json file.
fn get_assets_string(assets: Option<Vec<&str>>) -> String {
    let mut assets_string: String = String::new();
    if assets.is_none() {
        for (_key, value) in MANIFEST_ASSETS.iter() {
            let asset_type = value.split('.').last().unwrap();
            if asset_type == "css" {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", value));
            } else if asset_type == "js" {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", value));
            }
        }
    } else {
        for value in assets.unwrap().into_iter() {
            let asset_type = value.split('.').last().unwrap();
            let asset = MANIFEST_ASSETS.get(value);
            if asset_type == "css" && asset.is_some() {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", asset.unwrap()));
            } else if asset_type == "js" && asset.is_some() {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", asset.unwrap()));
            }
        }
    }

    assets_string
}

/// Parse the assets from the manifest.json file (called once at startup)
fn parse_manifest_assets() -> HashMap<String, String> {
    let mut assets: HashMap<String, String> = HashMap::new();
    let manifest: Result<String, Error> = fs::read_to_string("dist/.vite/manifest.json");
    if manifest.is_ok() {
        let manifest: String = manifest.unwrap();
        let manifest_json: serde_json::Value = serde_json::from_str(&manifest).expect("Failed to parse manifest.json");

        for (key, value) in manifest_json.as_object().unwrap().iter() {
            let asset = value.get("file");
            if asset.is_some() {
                assets.insert(key.to_string(), asset.unwrap().as_str().unwrap().parse().unwrap());

                // If the asset is a js file, we might add css files to the assets.
                let asset_type = asset.unwrap().as_str().unwrap().split('.').last().unwrap();
                if asset_type == "js" {
                    if let Some(css_array) = value.get("css").and_then(|v| v.as_array()) {
                        for css_file in css_array {
                            let css_file_name = css_file.as_str().unwrap().split('/').last().unwrap();
                            assets.insert(css_file_name.to_string(), css_file_name.to_string());
                        }
                    }
                }
            }
        }
    }

    assets
}
