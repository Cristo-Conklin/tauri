use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BundleTarget {
  All(Vec<String>),
  One(String),
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DebConfig {
  pub depends: Option<Vec<String>>,
  #[serde(default)]
  pub use_bootstrapper: bool,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OsxConfig {
  pub frameworks: Option<Vec<String>>,
  pub minimum_system_version: Option<String>,
  pub exception_domain: Option<String>,
  pub license: Option<String>,
  #[serde(default)]
  pub use_bootstrapper: bool,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BundleConfig {
  /// Whether we should build your app with tauri-bundler or plain `cargo build`
  pub active: bool,
  /// The bundle targets, currently supports ["deb", "osx", "msi", "appimage", "dmg"] or "all"
  pub targets: Option<BundleTarget>,
  pub name: Option<String>,
  /// The app's identifier
  pub identifier: Option<String>,
  /// The app's icons
  pub icon: Option<Vec<String>>,
  pub version: Option<String>,
  /// App resources to bundle.
  /// Each resource is a path to a file or directory.
  /// Glob patterns are supported.
  pub resources: Option<Vec<String>>,
  pub copyright: Option<String>,
  pub category: Option<String>,
  pub short_description: Option<String>,
  pub long_description: Option<String>,
  pub script: Option<PathBuf>,
  #[serde(default)]
  pub deb: DebConfig,
  #[serde(default)]
  pub osx: OsxConfig,
  pub external_bin: Option<Vec<String>>,
}

/// A CLI argument definition
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CliArg {
  /// The short version of the argument, without the preceding -.
  ///
  /// NOTE: Any leading - characters will be stripped, and only the first non - character will be used as the short version.
  pub short: Option<char>,
  /// The unique argument name
  pub name: String,
  /// The argument description which will be shown on the help information.
  /// Typically, this is a short (one line) description of the arg.
  pub description: Option<String>,
  /// The argument long description which will be shown on the help information.
  /// Typically this a more detailed (multi-line) message that describes the argument.
  pub long_description: Option<String>,
  /// Specifies that the argument takes a value at run time.
  ///
  /// NOTE: values for arguments may be specified in any of the following methods
  /// - Using a space such as -o value or --option value
  /// - Using an equals and no space such as -o=value or --option=value
  /// - Use a short and no space such as -ovalue
  pub takes_value: Option<bool>,
  /// Specifies that the argument may appear more than once.
  ///
  /// - For flags, this results in the number of occurrences of the flag being recorded.
  /// For example -ddd or -d -d -d would count as three occurrences.
  /// - For options there is a distinct difference in multiple occurrences vs multiple values.
  /// For example, --opt val1 val2 is one occurrence, but two values. Whereas --opt val1 --opt val2 is two occurrences.
  pub multiple: Option<bool>,
  /// specifies that the argument may appear more than once.
  pub multiple_occurrences: Option<bool>,
  ///
  pub number_of_values: Option<u64>,
  /// Specifies a list of possible values for this argument.
  /// At runtime, the CLI verifies that only one of the specified values was used, or fails with an error message.
  pub possible_values: Option<Vec<String>>,
  /// Specifies the minimum number of values for this argument.
  /// For example, if you had a -f <file> argument where you wanted at least 2 'files',
  /// you would set `minValues: 2`, and this argument would be satisfied if the user provided, 2 or more values.
  pub min_values: Option<u64>,
  /// Specifies the maximum number of values are for this argument.
  /// For example, if you had a -f <file> argument where you wanted up to 3 'files',
  /// you would set .max_values(3), and this argument would be satisfied if the user provided, 1, 2, or 3 values.
  pub max_values: Option<u64>,
  /// Sets whether or not the argument is required by default.
  ///
  /// - Required by default means it is required, when no other conflicting rules have been evaluated
  /// - Conflicting rules take precedence over being required.
  pub required: Option<bool>,
  /// Sets an arg that override this arg's required setting
  /// i.e. this arg will be required unless this other argument is present.
  pub required_unless_present: Option<String>,
  /// Sets args that override this arg's required setting
  /// i.e. this arg will be required unless all these other arguments are present.
  pub required_unless_present_all: Option<Vec<String>>,
  /// Sets args that override this arg's required setting
  /// i.e. this arg will be required unless at least one of these other arguments are present.
  pub required_unless_present_any: Option<Vec<String>>,
  /// Sets a conflicting argument by name
  /// i.e. when using this argument, the following argument can't be present and vice versa.
  pub conflicts_with: Option<String>,
  /// The same as conflictsWith but allows specifying multiple two-way conflicts per argument.
  pub conflicts_with_all: Option<Vec<String>>,
  /// Tets an argument by name that is required when this one is present
  /// i.e. when using this argument, the following argument must be present.
  pub requires: Option<String>,
  /// Sts multiple arguments by names that are required when this one is present
  /// i.e. when using this argument, the following arguments must be present.
  pub requires_all: Option<Vec<String>>,
  /// Allows a conditional requirement with the signature [arg, value]
  /// the requirement will only become valid if `arg`'s value equals `${value}`.
  pub requires_if: Option<Vec<String>>,
  /// Allows specifying that an argument is required conditionally with the signature [arg, value]
  /// the requirement will only become valid if the `arg`'s value equals `${value}`.
  pub required_if_eq: Option<Vec<String>>,
  /// Requires that options use the --option=val syntax
  /// i.e. an equals between the option and associated value.
  pub require_equals: Option<bool>,
  /// The positional argument index, starting at 1.
  ///
  /// The index refers to position according to other positional argument.
  /// It does not define position in the argument list as a whole. When utilized with multiple=true,
  /// only the last positional argument may be defined as multiple (i.e. the one with the highest index).
  pub index: Option<u64>,
}

/// describes a CLI configuration
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CliConfig {
  /// command description which will be shown on the help information
  description: Option<String>,
  /// command long description which will be shown on the help information
  long_description: Option<String>,
  /// adds additional help information to be displayed in addition to auto-generated help
  /// this information is displayed before the auto-generated help information.
  /// this is often used for header information
  before_help: Option<String>,
  /// adds additional help information to be displayed in addition to auto-generated help
  /// this information is displayed after the auto-generated help information
  /// this is often used to describe how to use the arguments, or caveats to be noted.
  after_help: Option<String>,
  /// list of args for the command
  args: Option<Vec<CliArg>>,
  /// list of subcommands of this command.
  ///
  /// subcommands are effectively sub-apps, because they can contain their own arguments, subcommands, usage, etc.
  /// they also function just like the app command, in that they get their own auto generated help and usage
  subcommands: Option<HashMap<String, CliConfig>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Port {
  /// Port with a numeric value.
  Value(u16),
  /// Random port.
  Random,
}

/// The embeddedServer configuration object.
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmbeddedServerConfig {
  /// The embedded server host.
  pub host: Option<String>,
  /// The embedded server port.
  /// If it's `random`, we'll generate one at runtime.
  pub port: Option<Port>,

  /// The base path of the embedded server.
  /// The path should always start and end in a forward slash, which the deserializer will ensure
  pub public_path: Option<String>,
}

/// The window configuration object.
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WindowConfig {
  /// The window identifier.
  pub label: Option<String>,
  /// The window webview URL.
  pub url: Option<String>,
  /// The horizontal position of the window's top left corner
  pub x: Option<f64>,
  /// The vertical position of the window's top left corner
  pub y: Option<f64>,
  /// The window width.
  pub width: Option<f64>,
  /// The window height.
  pub height: Option<f64>,
  /// The min window width.
  pub min_width: Option<f64>,
  /// The min window height.
  pub min_height: Option<f64>,
  /// The max window width.
  pub max_width: Option<f64>,
  /// The max window height.
  pub max_height: Option<f64>,
  /// Whether the window is resizable or not.
  #[serde(default)]
  pub resizable: bool,
  /// The window title.
  pub title: Option<String>,
  /// Whether the window starts as fullscreen or not.
  #[serde(default)]
  pub fullscreen: bool,
  /// Whether the window is transparent or not.
  #[serde(default)]
  pub transparent: bool,
  /// Whether the window is maximized or not.
  #[serde(default)]
  pub maximized: bool,
  /// Whether the window is visible or not.
  #[serde(default)]
  pub visible: bool,
  /// Whether the window should have borders and bars.
  #[serde(default)]
  pub decorations: bool,
  /// Whether the window should always be on top of other windows.
  #[serde(default)]
  pub always_on_top: bool,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecurityConfig {
  csp: Option<String>,
}

/// The Tauri configuration object.
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TauriConfig {
  /// The windows configuration.
  #[serde(default)]
  pub windows: Vec<WindowConfig>,
  /// The CLI configuration.
  pub cli: Option<CliConfig>,
  #[serde(default)]
  pub embedded_server: EmbeddedServerConfig,
  /// The bundler configuration.
  #[serde(default)]
  pub bundle: BundleConfig,
  #[serde(default)]
  pub allowlist: HashMap<String, bool>,
  pub security: Option<SecurityConfig>,
}

/// The Build configuration object.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BuildConfig {
  /// the app's dev server URL, or the path to the directory containing an index.html file
  #[serde(default = "default_dev_path")]
  pub dev_path: String,
  /// the path to the app's dist dir. This path must contain your index.html file.
  #[serde(default = "default_dist_dir")]
  pub dist_dir: String,
  /// a shell command to run before `tauri dev` kicks in
  pub before_dev_command: Option<String>,
  /// a shell command to run before `tauri build` kicks in
  pub before_build_command: Option<String>,
  /// Whether we should inject the Tauri API on `window.__TAURI__` or not.
  #[serde(default)]
  pub with_global_tauri: bool,
}

fn default_dev_path() -> String {
  "".to_string()
}

fn default_dist_dir() -> String {
  "../dist".to_string()
}

type JsonObject = HashMap<String, JsonValue>;

/// The tauri.conf.json mapper.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
  /// The Tauri configuration.
  #[serde(default)]
  pub tauri: TauriConfig,
  /// The build configuration.
  #[serde(default = "default_build")]
  pub build: BuildConfig,
  /// The plugins config.
  #[serde(default)]
  pub plugins: HashMap<String, JsonObject>,
}

fn default_build() -> BuildConfig {
  BuildConfig {
    dev_path: default_dev_path(),
    dist_dir: default_dist_dir(),
    before_dev_command: None,
    before_build_command: None,
    with_global_tauri: false,
  }
}
