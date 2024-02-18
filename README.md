# KeyBoard Layout analYZER
Analyze and get a feel of a different keyboard layout from your layout

## Configuration
Change the configuration by overwriting the default layout with your layout (located here `~/layouts/default.conf`)

## Testing a layout
Make sure to configure your layout before using `kblyzer`
Build the project using the following command:
```
  cargo build --release
```
Test a layout using the following command:
```
  cargo run [layout-name]
```

## Adding custom layouts
You can add your own layouts here `~/layouts/some-layout-name`

## Wordlists
This wordlist is taken from [monkeytype](https://github.com/monkeytypegame/monkeytype/blob/master/frontend/static/languages/english_1k.json).
All the words in the wordlist is sorted according to frequency
