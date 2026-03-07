# Junk

Junk is a small data language for describing game entities in an
easier-to-write format that maps directly to JSON. It has a limited syntax to
suit my needs at this exact moment, but I'm open to extending it with more
functionality.

## Usage

```
junk [OPTIONS] <FILES>...
```

Every input `.junk` file is translated into a matching `.json` file using the
same filename stem. Output goes to the current directory unless specified
otherwise with `--output`/`-o`.

```sh
junk entities.junk items.junk -o out/
```

## Format

A `.junk` must contain either a sequence of values _or_ a sequence of key-value
definitions. Values and definitions must be newline-separated; commas are not
permitted. See the list and object explanations below for more details.

Line comments start with `//`. Block comments are not supported.

_// FIXME add block comments_

### Values

**Literals.** The primitive types are booleans, integers, floats, and strings.

```
true
false
42
-7
3.14
-0.5
"hello world"
```

Integers and floats follow standard rules: a leading minus sign is allowed, and
leading zeros are not (so `07` is invalid). Floats require digits on both sides
of the decimal point (so `1.` and `.5` are invalid).

_// FIXME add other formatting options for hex and binary literals_

Strings are double-quoted. There are no escape sequences — if you need a
literal double-quote inside a string, you're out of luck.

_// FIXME add more string support here_

**Lists.** A sequence of values wrapped in `[...]`. Items must be _either_
newline-separated or comma-separated. Trailing commas are not allowed.

```
[1, 2, 3]
["warrior", "mage", "rogue"]
[
    [1, 2]
    [3, 4]
]
```

**Objects.** A sequence of key-value definitions wrapped in `{...}`. Definitons
must be _either_ newline-separated or comma-separated. Trailing commas are not
allowed.

Definitions may be a `key: value` pair or a boolean shorthand of `key` or
`!key`. Keys are alphanumeric and may contain periods, hyphens, or underscores.

An optional `#id` may be given before the opening brace; this is added as the
`"id"` key in the JSON output.

```
#long-sword {
    damage: 15
    type: "slashing"
    two-handed
    !stackable
}
```

## JSON mapping

Each construct maps to a JSON equivalent:

| Junk | JSON |
|---|---|
| `true` / `false` | boolean |
| `42`, `-7` | number |
| `3.14`, `-0.5` | number |
| `"text"` | string |
| `[a, b, c]` | array |
| `#name { ... }` | object with `"id"` as the first key |
| `flag` | `"flag": true` |
| `!flag` | `"flag": false` |

So this input:

```
#health-potion {
    heal: 50
    consumable
    !stackable
    tags: ["item", "consumable"]
}
```

produces:

```json
[{"id": "health-potion", "heal": 50, "consumable": true, "stackable": false, "tags": ["item", "consumable"]}]
```

Output is written as a single line with no pretty-printing. Use `jq` if you
need pretty-printing.

## Building

```sh
cargo build --release
```
