use std::collections::HashMap;
use std::fs;
use std::io::Error;
use tera::{Context, Tera};

pub fn load_template(template: &str, params: Vec<(&str, &str)>, assets: Option<Vec<&str>>) -> String {
    let tera = match Tera::new("src/resources/views/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context: Context = Context::new();
    for (key, value) in params.into_iter().collect::<Vec<(&str, &str)>>() {
        context.insert(key, value);
    }

    if !context.contains_key("session_message") {
        context.insert("session_message", "");
    }

    context.insert("assets", &get_assets_string(assets));

    tera.render(template, &context).expect("Failed to render template")
}

// Here we return the html string to add the assets to the template.
// If the assets are passed, we only add the assets passed, otherwise we add all the assets from
// the manifest.json file.
fn get_assets_string(assets: Option<Vec<&str>>) -> String {
    let mut assets_string: String = String::new();
    if assets.is_none() {
        for (key, value) in get_manifest_assets().into_iter().enumerate() {
            let asset_type = value.1.split('.').last().unwrap();
            if asset_type == "css" {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", value.1));
            } else if asset_type == "js" {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", value.1));
            }
        }
    } else {
        let manifest_assets = get_manifest_assets();
        for (key, value) in assets.unwrap().into_iter().enumerate() {
            let asset_type = value.split('.').last().unwrap();
            let asset = manifest_assets.get(value);
            if asset_type == "css" && asset.is_some() {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", asset.unwrap()));
            } else if asset_type == "js" && asset.is_some() {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", asset.unwrap()));
            }
        }
    }

    assets_string
}

// Here we get the assets from the manifest.json file.
fn get_manifest_assets() -> HashMap<String, String> {
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
                    for css_file in value.get("css").unwrap().as_array().unwrap() {
                        let css_file_name = css_file.as_str().unwrap().split('/').last().unwrap();
                        assets.insert(css_file_name.to_string(), css_file_name.to_string());
                    }
                }
            }
        }
    }

    assets
}
