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

A `.junk` file is a flat sequence of values that's mapped into a JSON array.

Line comments start with `//`. Block comments are not supported.

_// FIXME add block comments_

### Values

**Literals.** The primitive types are booleans, integers, and strings.

_// FIXME add float support_

```
true
false
42
-7
"hello world"
```

Integers follow standard rules: a leading minus sign is allowed, and leading
zeros are not (so `07` is invalid).

_// FIXME add other formatting options for hex and binary literals_

Strings are double-quoted. There are no escape sequences — if you need a
literal double-quote inside a string, you're out of luck.

_// FIXME add more string support here_

**Lists.** Wrap any number of values in `[...]`. Items must be _either_
newline-separated or comma-separated. Trailing commas are not allowed.

```
[1, 2, 3]
["warrior", "mage", "rogue"]
[
    [1, 2]
    [3, 4]
]
```

**Objects.** This is the main building block. An object is a `#name { ... }`
expression: a hash sign, an identifier for the object's name, and a
brace-delimited body of definitions. Definitons must be _either_
newline-separated or comma-separated. Trailing commas are not allowed.

```
#long-sword {
    damage: 15
    type: "slashing"
    two-handed
    !stackable
}
```

The name becomes an `"id"` key in the JSON output, always inserted first.

_// FIXME allow anonymous objects_

### Object bodies

The body of an object holds definitions, optionally separated by commas. There are three forms:

| Syntax | Meaning |
|---|---|
| `key: value` | assign any value to `key` |
| `key` | shorthand for `key: true` |
| `!key` | shorthand for `key: false` |

Keys are alphanumeric with periods, hyphens, and underscores.

## JSON mapping

Each construct maps to a JSON equivalent:

| Junk | JSON |
|---|---|
| `true` / `false` | boolean |
| `42`, `-7` | number |
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
{"id": "health-potion", "heal": 50, "consumable": true, "stackable": false, "tags": ["item", "consumable"]}
```

Output is written as a single line with no pretty-printing. Use `jq` if you
need pretty-printing.

Since the top level of a `.junk` file is always a sequence, the output is
always a JSON array, even when the file contains only one value.

_// FIXME allow top-level objects instead of lists_

## Building

```sh
cargo build --release
```
