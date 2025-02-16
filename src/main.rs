mod default_settings;
mod images;
mod quotes;
mod settings;

extern crate clap;
use clap::Parser;
use image::{image_dimensions, ImageResult};
use images::list_files;
use quotes::{get_quote, Quote};
use rand::prelude::*;
use settings::{parse_config, print_config, NerdcliConfig};
use std::path::{Path, MAIN_SEPARATOR};
use viuer::{print_from_file, Config as ImageConfig};

use colored::Colorize;
use std::io::{stdout, Write};
use termion::color;
use termion::raw::IntoRawMode;

#[derive(Parser)]
#[command(name = "Nerd-CLI")]
#[command(version = "1.0")]
#[command(about = "Add a bit of nerdyness to your terminal", long_about = None)]
struct Cli {
    #[arg(short, long, help = "Sets the debug mode")]
    debug: bool,

    #[arg(
        short,
        long,
        help = "For what so ever debugging reasons: hide the image"
    )]
    no_image: bool,

    #[arg(
        short = 'x',
        long,
        help = "Overwrite the max_width_percentage from the config file"
    )]
    max_width_percentage: Option<u16>,

    #[arg(
        short = 'y',
        long,
        help = "Overwrite the max_height_percentage from the config file"
    )]
    max_height_percentage: Option<u16>,

    #[arg(
        short,
        long,
        help = "Overwrite the margin left of the image from the config file"
    )]
    left: Option<u16>,

    #[arg(
        short,
        long,
        help = "Overwrite the margin above the image from the config file"
    )]
    above: Option<i16>,

    #[arg(
        short = 's',
        long,
        help = "Overwrite the layout set in the config file: [ROW | ROW_CENTERED | COL | COL_CENTERED]"
    )]
    layout: Option<String>,

    #[arg(
        short = 'w',
        long,
        help = "Set a specific image (absolute path) that will be displayed. Good for testing layouts."
    )]
    image: Option<String>,
}

fn calculate_image_layout(
    image: &String,
    config: &NerdcliConfig,
    cli: &Cli,
    quote_line_length: u16,
) -> (Option<u32>, Option<u32>, f32, f32, u16, u16) {
    const MIN_QUOTE_WIDTH: f32 = 50 as f32;
    const QUOTE_MARGIN: f32 = 5 as f32;
    // const MIN_QUOTE_HEIGHT: f32 = 25 as f32;
    const PROMT_HEIGHT: f32 = 16 as f32; // TODO make this configurable, as some prompts are multiple lines in height

    let top_margin = if cli.above.is_some() {
        cli.above.unwrap()
    } else {
        config.margin_top.unwrap()
    };
    let left_margin = if cli.left.is_some() {
        cli.left.unwrap()
    } else {
        config.margin_left.unwrap()
    };

    let (terminal_cells_width, tch) = termion::terminal_size().unwrap();

    // the factor 2 is due to the fact, that each cell in the terminal is double the width of a cell
    // the -16 is due to the fact, that after the image a newline and the prompt is printed
    let terminal_cells_height = (tch as f32 - PROMT_HEIGHT - top_margin as f32) * 2 as f32;
    // println!("Terminal size: {}x{}", terminal_cells_width, terminal_cells_height);

    let dim: ImageResult<(u32, u32)> = image_dimensions(Path::new(&image));
    let (_width, _height, ratio) = match dim {
        Ok((w, h)) => {
            let r = (h as f32 / w as f32) as f32;
            // println!("Image dimensions: {}x{} px. Ratio: {}", w, h, r);

            (w, h, r)
        }
        Err(_) => (0u32, 0u32, 0f32),
    };

    let mut x: f32 = left_margin as f32;
    let y: f32 = top_margin as f32;

    let max_width_percentage: f32 = if cli.max_width_percentage.is_some() {
        cli.max_width_percentage.unwrap() as f32
    } else {
        config.max_width_percentage.unwrap() as f32
    };
    let max_height_percentage: f32 = if cli.max_height_percentage.is_some() {
        cli.max_height_percentage.unwrap() as f32
    } else {
        config.max_height_percentage.unwrap() as f32
    };

    let layout = if cli.layout.is_some() {
        cli.layout.clone().unwrap()
    } else {
        config.layout.clone().unwrap()
    };

    let _show_quotes = config.show_quotes;

    if config.show_quotes {
        let boundary_image_width: f32 =
            terminal_cells_width as f32 * max_width_percentage / 100 as f32; // this is not restricted by the quote width and margins, as the image might be smaller
        let boundary_image_height: f32 =
            terminal_cells_height as f32 * max_height_percentage / 100 as f32;

        if layout == "ROW" || layout == "ROW_CENTERED" {
            let mut current_image_width: f32 = boundary_image_width;
            let mut current_image_height = current_image_width as f32 * ratio;
            let mut current_quote_width = terminal_cells_width as f32 - current_image_width;

            // reduce the image size as long as the heigt is too high for the terminal or the quote would not fit
            while current_quote_width < MIN_QUOTE_WIDTH
                || current_image_height > terminal_cells_height as f32
                || current_image_height > (boundary_image_height - (y as f32 * 2 as f32))
                || (terminal_cells_width as f32)
                    < current_image_width
                        + MIN_QUOTE_WIDTH
                        + QUOTE_MARGIN
                        + left_margin as f32
                        + left_margin as f32
            {
                current_image_width -= 1 as f32;
                current_image_height -= 2 as f32;
                // current_image_height = current_image_width as f32 * ratio;
                current_quote_width = terminal_cells_width as f32 - current_image_width;
            }

            // calculate the y position of the quote:
            current_image_height = current_image_width as f32 * ratio - y as f32; // 5 sind die zwei Zeilen für den Prompt

            let quote_y = ((current_image_height as f32 / 2 as f32) - quote_line_length as f32)
                / 2 as f32
                + top_margin as f32;

            let quote_x = if layout == "ROW_CENTERED" {
                x = (terminal_cells_width as f32
                    - current_image_width
                    - MIN_QUOTE_WIDTH
                    - QUOTE_MARGIN)
                    / 2 as f32;
                current_image_width + x as f32 + QUOTE_MARGIN
            } else {
                // x is already left_margin
                current_image_width + x as f32 + QUOTE_MARGIN
            };

            return (
                Some(current_image_width as u32),
                None,
                x,
                y,
                quote_x as u16,
                quote_y as u16,
            );
        } else {
            let mut current_image_height: f32 = boundary_image_height;
            let mut current_image_width: f32 = current_image_height as f32 / ratio;
            // let mut current_quote_height = terminal_cells_height as f32  - current_image_height;

            while current_image_width > terminal_cells_width as f32
                || current_image_width > boundary_image_width
            {
                current_image_width -= 1 as f32;
                current_image_height -= 2 as f32;
            }

            // calculate the x position of the quote:
            current_image_width = current_image_height as f32 / ratio;

            let quote_x = terminal_cells_width as f32 / 2 as f32 - MIN_QUOTE_WIDTH / 2 as f32; //-(current_image_width as f32 - MIN_QUOTE_WIDTH as f32) / 2 as f32 + x as f32;

            let quote_y = if layout == "COL_CENTERED" {
                x = (terminal_cells_width as f32 - current_image_width) / 2 as f32;
                current_image_height / 2 as f32 + y as f32 + QUOTE_MARGIN
            } else {
                // x is already top_margin
                current_image_height / 2 as f32 + y as f32 + QUOTE_MARGIN
            };

            return (
                Some(current_image_width as u32),
                None,
                x,
                y,
                quote_x as u16,
                quote_y as u16,
            );
        }
    } else if !config.show_quotes {
        //
        return (Some(0 as u32), None, 0 as f32, 0 as f32, 0 as u16, 0 as u16);
    }

    (Some(0 as u32), None, 0 as f32, 0 as f32, 0 as u16, 0 as u16)
}

fn calculate_quote_layout(quote: Quote, quote_box_width: u16) -> Vec<String> {
    // split the quote into lines only in word breaks and only if one line is longer than quote_box_width
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let mut current_line_length = 0;

    for natural_line in quote.text.split("\n") {
        for word in natural_line.split_whitespace() {
            if current_line_length + word.len() > quote_box_width as usize {
                lines.push(current_line.clone());
                current_line.clear();
                current_line_length = 0;
            }
            current_line.push_str(word);
            current_line.push_str(" ");
            current_line_length += word.len() + 1;
        }
        lines.push(current_line.clone());
        current_line.clear();
        current_line_length = 0;
    }
    lines
}

fn print_quote(
    quote_in_lines: Vec<String>,
    quote: Quote,
    quote_x: u16,
    quote_y: u16,
    settings: &NerdcliConfig,
) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    for (i, line) in quote_in_lines.iter().enumerate() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(quote_x, quote_y + i as u16),
            termion::clear::UntilNewline
        )
        .unwrap();
        println!(
            "{}",
            line.truecolor(
                settings.quote_color.r,
                settings.quote_color.g,
                settings.quote_color.b
            )
        );
    }

    println!("");

    let max_line_length = quote_in_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0) as u16;

    let source = format!(
        "{} {}",
        quote.source.unwrap_or("".to_string()),
        quote.date.unwrap_or("".to_string())
    );

    if source.len() > 0 {
        write!(
            stdout,
            "{} {}",
            termion::cursor::Goto(
                quote_x + max_line_length - 1 - source.len() as u16 - 1,
                quote_y + quote_in_lines.len() as u16 + 2
            ),
            termion::clear::UntilNewline
        )
        .unwrap();
        println!(
            "{}",
            source.italic().truecolor(
                settings.source_color.r,
                settings.source_color.g,
                settings.source_color.b
            )
        );
    }

    let author = quote.author;
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(
            quote_x + max_line_length - 1 - author.len() as u16 - 3,
            quote_y + quote_in_lines.len() as u16 + 3
        ),
        termion::clear::UntilNewline
    )
    .unwrap();
    println!(
        "-- {}",
        author.truecolor(
            settings.author_color.r,
            settings.author_color.g,
            settings.author_color.b
        )
    );

    // reset
    println!("{}", color::Fg(color::Reset));
}

fn main() {
    let cli = Cli::parse();

    let config = parse_config();

    print!("{}[2J", 27 as char); // Clear the terminal

    let image_path = [
        config.config_base_path.clone().unwrap(),
        config.image_dir.clone(),
    ]
    .join(&MAIN_SEPARATOR.to_string());

    let allimages: Vec<String> = list_files(Path::new(&image_path), &config)
        .unwrap()
        .iter()
        .map(|x| x.to_string_lossy().into_owned())
        .collect();

    // image is either a manually set path, a selected path from the folders or "".
    let image = if cli.image.is_some() {
        cli.image.clone().unwrap()
    } else {
        let mut rng = rand::rng();
        let selected_image = allimages.choose(&mut rng);
        if selected_image.is_none() {
            println!("No images found in the directory");
            "".to_string()
        } else {
            selected_image.unwrap().to_string()
        }
    };

    let quote_path = [
        config.config_base_path.clone().unwrap(),
        config.quotes_dir.clone(),
    ]
    .join(&MAIN_SEPARATOR.to_string());
    let quote = get_quote(Path::new(&quote_path), &config);
    let quote_in_lines = calculate_quote_layout(quote.clone(), 50 as u16);

    let (w, h, x, y, quote_x, quote_y) = calculate_image_layout(
        &image,
        &config,
        &cli,
        quote_in_lines.len() as u16 + 3 as u16,
    );
    if !cli.no_image {
        // build image conf from nerdcli config
        let image_conf = ImageConfig {
            // Set dimensions.
            width: w,    // TODO calculate / evtl cmp::max
            height: h, // if cli.height.is_some() {cli.height} else {config.height}, // TODO calculate either width or height
            x: x as u16, // if cli.left.is_some() {cli.left.unwrap()} else {config.margin_left.unwrap()},
            y: y as i16, //if cli.above.is_some() {cli.above.unwrap()} else {config.margin_top.unwrap()},
            ..Default::default()
        };

        print_quote(quote_in_lines.clone(), quote, quote_x, quote_y, &config);

        print_from_file(image.clone(), &image_conf).expect("Image printing failed.");

        let is_column_layout = cli.layout == Some("COL".to_string())
            || cli.layout == Some("COL_CENTERED".to_string())
            || config.layout == Some("COL".to_string())
            || config.layout == Some("COL_CENTERED".to_string());

        if is_column_layout {
            for _ in 1..=10 + quote_in_lines.len() {
                println!("");
            }
        }
        println!("");
    }

    if cli.debug {
        println!("*** DEBUG INFORMATION ***");
        println!("\nSelected image: {}", image);
        print_config(&config, &cli);

        println!("\nFound the following images: ");
        for name in allimages {
            println!("\t*{}", name);
        }
    }

    println!("");
}
