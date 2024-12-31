use std::error::Error;

use serde_json::Value;

pub fn get_uuid(name: &str) -> Result<String, Box<dyn Error>> {
  let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
  let response = ureq::get(&url).call()?;
  let body = response.into_string()?;
  let json: Value = serde_json::from_str(&body)?;

  match json.get("id") {
    Some(id) => Ok(id.to_string()),
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
