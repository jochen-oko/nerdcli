use crate::default_settings::ask_for_config_creation;
use crate::quotes::Color;
use crate::Cli;
use directories::ProjectDirs;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize, Default, Clone)]
pub struct NerdcliConfig {
    pub max_width_percentage: Option<u16>,
    pub max_height_percentage: Option<u16>,
    pub layout: Option<String>,
    pub show_quotes: bool,
    pub margin_left: Option<u16>,
    pub margin_top: Option<i16>,
    pub image_dir: String,
    pub quotes_dir: String,
    pub quote_languages: Vec<String>,
    pub quote_color: Color,
    pub source_color: Color,
    pub author_color: Color,
    pub image_types: Vec<String>,
    pub include_folders: Vec<String>,
    pub config_base_path: Option<String>,
}

fn print_option<T, A>(name: &str, x: Option<T>, a: Option<A>)
where
    T: std::fmt::Debug,
    A: std::fmt::Debug,
{
    if let Some(x) = x {
        println!(
            "\t{name}: {x:?} {ov}",
            name = name,
            x = x,
            ov = if let Some(a) = a {
                format!("\t(overwritten: {a:?})", a = a)
            } else {
                "".to_string()
            }
        );
    } else {
        println!(
            "\t{name}: None {ov}",
            ov = if let Some(a) = a {
                format!("\t(overwritten: {a:?})", a = a)
            } else {
                "".to_string()
            }
        );
    }
}

pub fn print_config(data: &NerdcliConfig, cli: &Cli) {
    let none: Option<String> = None;

    println!("\nNerd-CLI configuration:");
    print_option::<String, String>(
        "\tConfig and content path: {}",
        data.config_base_path.clone(),
        none,
    );

    println!("\n\tImage settings:");
    print_option(
        "max_width_percentage",
        data.max_width_percentage,
        cli.max_width_percentage.clone(),
    );
    print_option(
        "max_height_percentage",
        data.max_height_percentage,
        cli.max_height_percentage.clone(),
    );
    print_option("margin top", data.margin_top, cli.above.clone());
    print_option("margin left", data.margin_left, cli.left.clone());

    println!("\n\tQuote settings:");
    println!("\tshow_quotes: {:?}", data.show_quotes);

    println!("\n\tLayout settings:");
    print_option("layout", data.layout.clone(), cli.layout.clone());

    println!("\n\n\tContent settings:");
    println!("\timage_dir: {:?}", data.image_dir.clone());
    println!("\tquotes_dir: {:?}", data.quotes_dir.clone());
    println!("\timage_types: {:?}", data.image_types.join(", "));
    println!("\tinclude_folders: {:?}", data.include_folders.join(", "));
}

pub fn parse_config() -> NerdcliConfig {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "jo", "nerdcli") {
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App

        let config_file = proj_dirs.config_dir().join("nerdcli.toml");
        let config_string = proj_dirs
            .config_dir()
            .join("nerdcli.toml")
            .into_os_string()
            .into_string()
            .unwrap();

        let contents = match fs::read_to_string(config_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Could not read config file from `{}`: Error: {}",
                    config_string, e
                );
                ask_for_config_creation(proj_dirs);
                exit(1);
            }
        };
        match toml::from_str::<NerdcliConfig>(&contents) {
            Ok(d) => {
                let mut res = d.clone();
                res.config_base_path = Some(
                    proj_dirs
                        .config_dir()
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
                return res;
            }

            Err(e) => {
                eprintln!("Unable to parse config file `{}`: {}", config_string, e);
                exit(1);
            }
        };
    } else {
        eprintln!("Could not read config file.",);

        eprintln!("Depending on your OS, the config file must be located in a certain position (supposing your username is fry)");
        eprintln!("\tmacOS:   /Users/fry/Library/Application Support/dev.jo.nerdcli");
        eprintln!("\tLinux:   /home/fry/.config/nerdcli");
        eprintln!("\tWindows:   C:\\Users\\fry\\AppData\\Roaming\\jo\\nerdcli");
        exit(1);
    }
}
