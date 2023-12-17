<p align="center">
    <img src="./docs/images/logo.png" width="120" height="120" alt="logo">
</p>

<p align="center">
    <i>The icon are based on 
    <a href="https://prazdevs.github.io/pinia-plugin-persistedstate/">pinia-plugin-persistedstate</a>
    and 
    <a href="https://pinia.vuejs.org/">pinia</a></i>
</p>

<p align="center">
  A simple configuration file read-write library
</p>

<h1 align="center">persisted-config-rs</h1>

<p align="center">
  <a href="https://www.npmjs.com/package/pinia-plugin-persistedtauri">
    <img alt="npm" src="https://img.shields.io/crates/v/persisted-config-rs?color=%23c12127&label=persisted-config-rs&logo=rust" />
  </a>
  <a href="https://github.com/xiaochen0517/pinia-plugin-persistedtauri/blob/master/LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/xiaochen0517/persisted-config-rs?color=%233da639&logo=open%20source%20initiative" />
  </a>
</p>

## ✨ Quickstart

### 🚚 Install

```bash
cargo add persisted-config-rs
```

### 📦 Usage

```rust
use persisted_config_rs::PersistedConfig;

#[tauri::command]
fn set_item(key: String, value: String) {
    let mut config = PersistedConfig::new("config");
    config.set_item(key, value);
}

#[tauri::command]
fn get_item(key: String) -> String {
    let config = PersistedConfig::new("config");
    config.get_item(key)
}
```

## License

[MIT](./LICENSE)