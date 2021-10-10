# Parsing library and CLI tool for World Of Tanks replay files

The repository contains the library and CLI tool to load and parse metadata from `.wotreplay` files.

## `wot-replays` CLI tool

Sources: [wot-replays-cli](wot-replays-cli)

The tool allows to load and output metadata (battle
info and battle results). For example, the following command prints battle info from the
`wot-replays-lib/test_data/20210412_2144_ussr-R158_LT_432_01_karelia.wotreplay` file in JSON-format:

```bash
$ wot-replays parse -i \
    wot-replays-lib/test_data/20210412_2144_ussr-R158_LT_432_01_karelia.wotreplay
```

You can combine the command with
[`jq`](https://stedolan.github.io/jq/) to navigate thgrouh the JSON,
for example the following script outputs the annonimaizer's fake name:

```
$ wot-replays parse -i \
    wot-replays-lib/test_data/20210412_2144_ussr-R158_LT_432_01_karelia.wotreplay \
    | jq '.playerName as $playerName | .vehicles[] | select(.name == $playerName) | .fakeName'
```

## `wot-replays` rust library

Sources: [wot-replays-lib](wot-replays-lib)