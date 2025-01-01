use std::error::Error;

use base64::{prelude::BASE64_STANDARD, Engine};
use serde::Deserialize;
use serde_json::Value;

use super::SkinUrls;

#[derive(Deserialize)]
struct SkinResult {
  textures: SkinTextures,
}

#[derive(Deserialize)]
struct SkinTextures {
  #[serde(rename = "SKIN")]
  skin: Skin,
  #[serde(rename = "CAPE")]
  cape: Skin,
}

#[derive(Deserialize)]
struct Skin {
  url: String,
}

pub fn get_uuid(name: &str) -> Result<String, Box<dyn Error>> {
  let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
  let response = ureq::get(&url).call()?;
  let body = response.into_string()?;
  let json: Value = serde_json::from_str(&body)?;

  match json.get("id") {
    Some(id) => Ok(id.to_string().replace('"', "")),
    None => Err(format!("No uuid found for name {}", name).into()),
  }
}

pub fn get_name(uuid: &str) -> Result<String, Box<dyn Error>> {
  let url = format!("https://api.mojang.com/user/profiles/{}/names", uuid);
  let response = ureq::get(&url).call()?;
  let body = response.into_string()?;
  let json: Value = serde_json::from_str(&body)?;

  match json.get("name") {
    Some(name) => Ok(name.to_string()),
    None => Err(format!("No name found for uuid {}", uuid).into()),
  }
}

pub fn get_skin_data(uuid: &str) -> Result<SkinUrls, Box<dyn Error>> {
  let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", uuid);
  let response = ureq::get(&url).call()?;
  let body = response.into_string()?;
  let json: Value = serde_json::from_str(&body)?;
  
  let b64 = match json.get("properties").and_then(|p| p[0].get("value")) {
    Some(value) => value.to_string().replace('"', ""),
    None => return Err(format!("No skin found for uuid {}", uuid).into()),
  };

  let decoded = BASE64_STANDARD.decode(b64.as_bytes())?;
  let decoded = std::str::from_utf8(&decoded)?;
  let json: SkinResult = serde_json::from_str(decoded)?;

  Ok(SkinUrls {
    skin: json.textures.skin.url,
    cape: json.textures.cape.url,
  })
}