> [!WARNING]  
> Code may sporadically change. Nothing is set in stone
# Installation
## ~~Package Control~~ (when its ready)
- Open the Command Palette with `Ctrl/Cmd + Shift + P`
- Choose `Package Control: Install Package`
- Search and select `Hull Theme`

## Manual
- Clone this repository into your packages directory
	- macOS: `cd '~/Library/Application Support/Sublime Text/Packages'`
	- Windows: `cd '%appdata%/Sublime Text/Packages'`

## Theme Variants
There are various styles to sate as many corners of the palette.

| Variant        | Description                                                                                         |
|:---------------|:----------------------------------------------------------------------------------------------------|
| Bold (todo)    | An increase in opacity for outlines and backgrounds. This does not effect text                      |
| Bordered       | A distinct outline around elements                                                                  |
| Default        | The base theme                                                                                      |
| Sharp (todo)   | Pointy corners                                                                                      |
| Timid (todo)   | No indicators, borders or accents, increased background alpha values and reduced shadows            |
| Sublime (todo) | A blend between Sublime and Hull. Default font size, git status, file status and folder asset icons |
>Hull being an *adaptive* theme suggests that it should work with most color schemes (maybe not [*Hot Dog Stand*](https://github.com/SomeKittens/ST-Hot-Dog-Stand)). Settings can be flipped to bring the best out of ones color scheme.

### Enable a Theme
- Open the Command Palette with `Ctrl/Cmd + Shift + P`
- Search and select `UI: Select Theme`
- Search `Hull` and select one of the Variants

## Theme Options

The available theme variants cover a range styling dynamics. The toggles shown [~~here~~]() can be adjusted to suit your preference.

## Are there Color Schemes?

The first graphic on this page were some early overrides to Mariana (Built in) but didn't meet my preferences. There are so many different color schemes that its become fatigue inducing to find the perfect combo/s. I frequently switch between Mariana, a modded [Catppuccin](https://github.com/sukinoverse/catppuccin-sublime-text)  (different from the [original](https://github.com/catppuccin/sublime-text)) and [rsms](https://github.com/rsms/sublime-theme?tab=readme-ov-file) so it may never happen.

## More Options
For ***fine***-grained customisation, visit [**~~the documentation~~**]() or have a peek at the json [~~here~~](). I've also curated a list of possible encounters you may question, be sure to read [this](docs/Miscellaneous%20Notes.md) if you're interested.

## Project Plan
- Refine method structures (will probably need a rewrite)
- Restructure json to mimic something similar to [lottie](https://github.com/LottieFiles/lottie-docs/blob/main/docs/layers.md)
- Use [json-scheme](https://github.com/json-schema-org/json-schema-spec/blob/main/jsonschema-core.md#introduction) so we can avoid manual commenting, but would require LSP-json for assistance
- Error handling
- Add tests
- Move generation logic into a library crate
	- Move into another repo and pull in a binary
	- Create CLI endpoints

## Inspiration / Credits
- [Theme-DAneo](https://github.com/SublimeText/Theme-DAneo)
- [ayu](https://github.com/dempfi/ayu/)
- [dot-icons](https://github.com/anweber/dot-icons)
- Bundled Mariana Color Scheme (Sublime HQ)
- Bundled Adaptive Theme (Sublime HQ)
