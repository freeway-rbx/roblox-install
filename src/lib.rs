#![deny(clippy::all)]

use napi_derive::napi;

use roblox_install::RobloxStudio;

#[napi]
pub fn studio_plugins_path() -> String {
  let studio = RobloxStudio::locate().unwrap();
  let plugins = studio.plugins_path();

  let result = plugins.to_str().unwrap();

  return result.to_string();
}

#[napi]
pub fn studio_content_path() -> String {
  let studio = RobloxStudio::locate().unwrap();
  let content = studio.content_path();

  let result = content.to_str().unwrap();

  return result.to_string();
}
