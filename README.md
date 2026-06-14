# dprint-plugin-tex-fmt

A [dprint](https://dprint.dev) plugin that formats TeX files using [tex-fmt](https://github.com/WGUNDERWOOD/tex-fmt).

## Usage

```sh
dprint add kjanat/tex-fmt
dprint fmt
```

Or add manually to `dprint.json`:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/kjanat/tex-fmt-0.1.0.wasm"
  ]
}
```

## Configuration

All options are optional. Keys use `camelCase` under the `texFmt` top-level key. Global dprint options (`lineWidth`,
`indentWidth`, `useTabs`) are used as fallbacks for `wraplen`, `tabsize`, and `tabchar` respectively.

```json
{
  "texFmt": {
    "wrap": true,
    "wraplen": 80,
    "wrapmin": 70,
    "tabsize": 2,
    "tabchar": "space",
    "lists": [],
    "verbatims": [],
    "noIndentEnvs": [],
    "wrapChars": [" "],
    "formatTables": false
  }
}
```

| Key            | Type       | Default             | Description                                                               |
| -------------- | ---------- | ------------------- | ------------------------------------------------------------------------- |
| `wrap`         | `boolean`  | `true`              | Wrap long lines                                                           |
| `wraplen`      | `integer`  | `80` / `lineWidth`  | Maximum line length before wrapping                                       |
| `wrapmin`      | `integer`  | `wraplen - 10`      | Lines longer than this are wrapped (defaults to `wraplen - 10` when ≥ 50) |
| `tabsize`      | `integer`  | `2` / `indentWidth` | Spaces per indent level                                                   |
| `tabchar`      | `string`   | `"space"`           | Indentation character — `"space"` or `"tab"` (follows `useTabs`)          |
| `lists`        | `string[]` | built-in defaults   | Extra list environments                                                   |
| `verbatims`    | `string[]` | built-in defaults   | Extra verbatim environments                                               |
| `noIndentEnvs` | `string[]` | `["document"]`      | Environments that are not indented                                        |
| `wrapChars`    | `string[]` | `[" "]`             | Characters after which lines may be wrapped (single-character strings)    |
| `formatTables` | `boolean`  | `false`             | Enable experimental table formatting                                      |

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
