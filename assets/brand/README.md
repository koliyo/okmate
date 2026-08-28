# OKMate brand assets

The mark represents two peers meeting around a shared piece of knowledge:
the name's “OK, mate” origin without a literal chat symbol.

| Asset | Use |
| --- | --- |
| `okmate-app-icon.png` | Full-bleed 1024×1024 master (square canvas, no mask) |
| `okmate-app-icon-macos.png` | Runtime Dock/window icon: 824pt squircle, radius 185.4, transparent corners |
| `okmate.icns` | macOS bundle icon (`iconutil` set: 16–1024 px, including @2x) |

macOS does not mask a traditional `.icns` the way iOS masks an asset catalog.
Regenerate the derived assets after changing the master:

```sh
uv run --with pillow assets/brand/generate_icons.py
```
| `okmate-mark.svg` | Light-background mark for documents and web material |
| `okmate-mark-dark.svg` | Dark-background mark |
| `okmate-logo.svg` | Horizontal logo lockup |

Palette:

- Rocci orange: `#E64B2F`
- Dark occlusion accent: `#B92F19`
- Charcoal: `#242424`
- Warm white: `#F8FFFB`

Keep the orange limited to the shared centre and avoid adding additional
colours to the mark.
