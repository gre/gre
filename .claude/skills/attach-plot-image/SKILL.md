---
name: attach-plot-image
description: Attach a photo of a plotted artwork to a plot example, resize it, generate thumbnail, and update the README with a description.
argument-hint: "<plot-number> <image-path>"
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
---

# Attach Plot Image

Attach a photo to plot example `$ARGUMENTS[0]` from the image at `$ARGUMENTS[1]`.

If no image path is provided as second argument, search for the image in common locations:
- Google Drive: `~/Library/CloudStorage/GoogleDrive-*/My Drive/`
- Look for recent photos matching the plot date from the README

## Steps

### 1. Validate the plot exists

Check that `plots/examples/$ARGUMENTS[0]/main.rs` and `plots/examples/$ARGUMENTS[0]/README.md` exist.

### 2. Copy and resize the image

Resize the source image to **1400px on the longest side**, quality 90, and save to:
```
public/images/plots/$ARGUMENTS[0].jpg
```

Use ImageMagick:
```bash
magick "$SOURCE" -resize 1400x1400\> -quality 90 "public/images/plots/$ARGUMENTS[0].jpg"
```

Verify the result is reasonable (typically 400-900KB for plot images).

### 3. Generate thumbnail

Generate a 512x512 max thumbnail:
```bash
magick "public/images/plots/$ARGUMENTS[0].jpg" -resize 512x512\> "public/images/plots/$ARGUMENTS[0]-thumbnail.jpg"
```

### 4. Update the README

Ensure the README.md frontmatter has:
- `image: /images/plots/$ARGUMENTS[0].jpg`
- A `description:` field in the frontmatter (one-line summary)

### 5. Add a body description

Read the `main.rs` to understand the algorithm, then write a short body description (2-3 sentences) after the frontmatter `---` explaining:
- What the artwork looks like visually
- The algorithm/technique used
- Any notable implementation details

### 6. Verify

Show the final README content and image file sizes to the user for confirmation.
