> [!NOTE]
> Some comments are from my very first version so it won't make sense until I eventually build upon it in my rewrite 
# A list of miscellaneous notes / issues you may encounter

### Global

- Resizable element areas (i.e. Panel, Sidebar) with negative space have the drawback of an incorrect perceived resize hover area. Pixels are offset a little so its very subtle but can get annoying if you're resizing often.  The space is used for shadows.

### Tab Control
- [`inactive_sheet_dimming`](https://www.sublimetext.com/docs/themes.html#inactive_sheet_dimming)  which was introduced in ST~~~4~~~ doesn't work well with this themes inherit design and so, is disabled by default even if set to `true`. You may enable this with [`"hull_inactive_sheet_dimming": true`]() but it isn't intended to look pleasing.
- Setting a different color scheme at the [project level](https://www.sublimetext.com/docs/projects.html#settings-key) settings may yield different design looks. This can be especially obvious when picking light/dark color scheme, i.e. the corners of a view will be boxy.

### Sidebar

- Content in the sidebar may escape the disguised corner textures. Its noticeable with rounded corners and increasingly more clear with higher values. Unfortunately there isn't a way to fix this, but to only reduce the roundness levels. There's enough x-axis margin space to give you enough room to play with supporting radius options.

--- 

- To make room for shadows, margins and `vcs` texture designs, `tree_row`s are slightly taller. `sidebar_tree``row_padding` has changed to `[n, 4, n, 5]` from `2` on Windows, `3` OSX and Linux. This allows room for the fancy glam, particularly shadow textures.

--- 

- The asset dimensions of some textures, offset the text in `tree_row` elements a little on `Windows`, the row padding is set to `[n, 4, n, 6]`. This is a slight increase from the padding on Linux and OSX since the text behaves differently from what seems to be centering inside the modal.

---

- Hovered / selected items may look cut off and travel beyond what a hovered/selected element may be sized as. To keep the style consistent use `"hull_sidebar_tree_row_margin_right": "none"`. Each `tree_row` element is stretched to the largest calculated modal of its kind, textures with `draw_center: true` will also extend with these dimensions. This behaviour can also be seen in [Ayu Theme](https://github.com/dempfi/ayu) and will most likely stay this way for performance reasons / customer value.  See [~~this~~]() issue for further info.

--- 

- Space between UI elements are generally filled with the color schemes background color. A few schemes like [Solarized Dark](https://github.com/braver/Solarized) have specified a gutter color that can break off from this design. You can create an override, Open the Command Palette with `Ctrl/Cmd + Shift + P`, then search and select `UI: Customize Color Scheme`. Copy the line containing `gutter` and paste it into the `globals` object. See below..
```jsonc
// Documentation at https://www.sublimetext.com/docs/color_schemes.html
{
  "globals":
  {
    "gutter": "var(--background)",
  },
}
```

#### VCS Icons

- When using the lowercase glyphs set, some glyph stems travel outside the center which can ruin the aesthetic. This is a trade off when using this set.
- There is currently only one size of `vcs` upper/lower case glyphs due to glyphs looking different at different scalings. Small and larger textures may eventually be added.
- Squircles assets will eventually be added,  edit the path builder and add the options.
- Possible curvature options: 1.0, 0.95, 0.9, 0.75, 0.5, 0.25, 0.

#### Scroll

- Hovering between the `sidebar_tree` and `view` would re-trigger the appearance of the scrollbar which I found as a small distraction. You can revert the option [~~here~~]().
- Hovering over the scrollbar will no longer show `track_control_area` element if `"overlay_scroll_bars"` is `true`, if false, it will show. You can revert the option [~~here~~]().

### Status Bar

- The `vcs_status` font previewed in the promo images is [JetBrains Mono](https://www.jetbrains.com/lp/mono/). Use []() to enable this, be sure that you have the font installed.

### Assets

- This theme is designed for both low and high DPI screens. To keep assets looking crisp, standard scaled assets may be different to their scaled up versions.

### Colors

- The sidebar vcs icons lean towards a forced color bias to ensure that they are visible with as many color schemes. For grey monochromatic schemes like ([`rsms-dark-mono`](https://github.com/rsms/sublime-theme/blob/main/rsms-dark-mono.sublime-color-scheme)), you may not want this bias. The `"hull_vcs_color": "direct"` option will instead use the colors directly from the active color scheme. Refer to [~~this link~~]() for other available options.

```jsonc
/*
  `bias`    Color Scheme + mod function adjusters
  `direct`  Color Scheme
  `css3`    Colors selected from CSS3 name specification
*/

"hull_vcs_colors": ["bias", "direct", "css3"]

```

### Opacity

- Almost all elements use translucent opacity levels (below 1.0), some even below 1 percent. When transitioning between two values that are less then `0.01`, a change of `0.005` can change easily effect the timing of an animation.

### File Icons
Folder colors are defaulted to orange to pair with Sublime's bundled icon file type set. You may want to change this using `xxx`. i.e. Option `"disclosure_icon_tint": "greyish"` pairs well with [FileIcons](https://github.com/braver/FileIcons) as folders won't not stick out, instead focusing attention towards the colorized icon file types.
