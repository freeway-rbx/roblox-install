extern crate napi_derive;
use napi_derive::napi;

use roblox_install::RobloxStudio;

#[napi]
pub fn studio_plugins_path() -> Option<String> {
  let studio = RobloxStudio::locate().ok()?;
  let plugins = studio.plugins_path();
  plugins.to_str().map(|s| s.to_string())
}

#[napi]
pub fn studio_content_path() -> Option<String> {
  let studio = RobloxStudio::locate().ok()?;
  let content = studio.content_path();
  content.to_str().map(|s| s.to_string())
}
