

# nerdcli
A command line tool to greet you with your favorite popculture references.

It all started with a bit of spare time and the interest in trying out rust.

Although it works on my machine (tm), I have no clue, if it works with different Terminal Emulators or Operating Systems. (I tested on arch and ubuntu with kitty).

# Installation
1. Clone this repo or download the source code
2. build with ```cargo build --release```
3. copy the executable ```nerdcli``` to the base path of the repo (so that it on the same level as the ```assets``` folder). ```nerdcli``` checks on startup if it can find the configuration. If no config exists, it will create a default configuration from the content of the ```assets``` folder.
4. If your config was created (automatically or manually), you can put the executable anywhere you want
5. call the ```nerdcli```-program from your shell-config file (e.g. for zsh under Linux: ~/.zshrc)

However the default configuration just contains some quotes and one image.

## Get more images
To get things to be a bit more personal, just keep your screenshot tool at hand while searching for your favourite images. It's for the CLI, so size and resolution is not so much of a deal breaker.
Copy new images to the directory ```images``` in the config directory. You can also organize images in sub-folders, as the ```images```-directory will be read recursively.

## Get more quotes
For more quotes, you can get manual or just ask chatGPT to provide some from author xy in the format:
[[quotes]]
text = ""
author = ""
date = ""
source = ""

Paste all quotes to an existing or new ```toml```-file in the config directory under quotes/[language]. If you put your quotes in a different directory than 'quotes/en', be sure to update ```quotes_dir``` and ```quote_language``` in the ```nerdcli.toml```.


# Impressions and configurations

![alt text](<screenshots/01.png>)
layout="ROW"
Art Copyright by Mike Mignola

![alt text](<screenshots/02.png>)
layout="ROW_CENTERED"
Art Copyright by Mike Mignola

![alt text](<screenshots/03.png>)
Art="COL_CENTERED"

![alt text](<screenshots/04.png>) 
Art Copyright by Mike Mignola

![alt text](<screenshots/05.png>)
Art Copyright by David Petersen

![alt text](<screenshots/06.png>)
Art Copyright by Chris Long

![alt text](<screenshots/07.png>)

![alt text](<screenshots/08.png>)

## Configurations in nerdcli.toml
### layout
* ROW: image left, near the border, quote to the right of the image.
* ROW_CENTERED: image and quote centered, image left, quote right
* COL: image left, quote underneath
* COL_CENTERED: image centered, quote centered underneath

### max_width_percentage and max_height_percentage
Defines how high/wide an image will be in maximum - relative to the terminal size.

### margin_top and margin_left
Defines the gap above the image and left of the image.

### image dir and quote dir
Defines the location of the images resp the quotes. The path must be relative to the config file nerdcli.toml.

### quote_languages
Defines, which languages will be selected for the quotes. Actually it's just the top level folder names in the quotes folder.

### colors
Colors can be set separately as RGB values for
* quotes
* source and date
* author

### image types
If only certain image types should be allowed, these can be filtered in image_types.
JPGs do take more time to load (on my machine) and gifs are not animated.

### include_folders
If only certain images should be included, these can be set in include_folders.
However, if comics/marvel should be included, then comics must be included as well (and this won't include other subdirectories in comics/, e.g. comics/mouseguard)


## Overwrite configuration in program arguments
nerdcli can handle some program arguments that overwrite settings from the config-file.
You can check the options with 

$> nerdcli -h

# Known Issues
The calculation of the correct height of the image differs a bit for different layouts.
So with layout="ROW_CENTERED", a percentaged height of 50 might be the same as a height of 90 with layout="COL". But this also changes a bit with different terminal sizes.
However with the same setup (same settings, same terminal size) it's consistent.

# Troubleshoot
## No image is showing
* first of all, check if the image was loaded from the configuration by running

```nerdcli -d```

It should output a selected image if everything worked well.
Then play around with the layout, max-height-percentage and max-width-percentage, e.g. by runngin

```nerdcli -sROW_CENTERED -x50 -y50```
Then try changing values until it suits you. See nerdcli -h for more config parameters.

Check the max_width and max_height settings as well as the 


## Unused configuration
* show_quotes currently has no effect.

## Next steps
* Option to show only images
* Option to show only quotes
* clean up code
* select quotes based on the image (not sure yet).
* better color management


# Build yourself
Clone the repo and run ```cargo build --release```. When running for the first time, make sure that the assets folder (on root-level in this repo) is in the same directory as the executable (which typically will be put to target/release). This way, the initial config as well as some quotes and an image is copied to the correct places.
