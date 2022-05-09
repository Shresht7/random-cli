# Random CLI

A command-line interface to do random stuff. Generate random strings and numbers, flip a coin, make decisions and resolve dilemmas. There is probably something in your life that can use a bit of randomness.

## Usage

```sh
random [COMMAND] [FLAGS]
```

Use the `help` command to see more details. `random help` or `random --help` or `random [COMMAND] --help`.

---

## Commands

| Command  | Description                           |
| -------- | ------------------------------------- |
| `select` | Select one entry from a list randomly |
| `number` | Generate a random number              |
| `string` | Generate a random string              |
| `toss`   | Toss a coin                           |
| `roll`   | Roll a die                            |

### Select

Select one entry from a given list randomly

```sh
random select [...ENTRIES]
```

#### Arguments

Takes a variable amount of arguments representing the list of all entries.

```sh
random select one two three four five ...
```

Can also accept input from a redirected stdin.

```sh
gh repo list | random select
```

#### Flags

You can use the `--repeat` flag to select multiple items from the list

```sh
gh repo list | random select --repeat 3
```

#### Examples

```sh
random select pizza burger pasta        # Selects either pizza, burger or pasta
gh repo list | random select            # Select one entry piped through stdin
```


### Number

Generate random numbers

```sh
random number [NUM1] [NUM2]
```

#### Arguments

`[NUM1]` and `[NUM2]` are optional positional parameters that determine the range within which to generate the number.

- If both `[NUM1]` and `[NUM2]` are specified, the command generates a number between `[NUM1]` and `[NUM2]`.
- If only `[NUM1]` is specified, the command generates a number between `0` and `[NUM1]`.
- If no parameters are specified, the command generates a number between `0` and `1000`.

#### Flags

You can use the `--repeat` flag to generate multiple random numbers

#### Examples

```sh
random number                   # Generates a number between 0 and 1000
random number 5                 # Generates a number between 0 and 5
random number 5 15              # Generates a number between 5 and 15
random number 5 10 --repeat 5   # Generate 5 numbers between 5 and 15     
```

### String

Generate random strings using alphanumeric and special characters

```sh
random string
```

#### Flags

You can use the `--length` flag to specify the length of the generated strings. (defaults to `16`)

You can use the `--repeat` flag to generate multiple random strings.

#### Examples

```sh
random string                           # Generates a random 16-character string
random string --length 8                # Generates a radom 8-character string
random string --length 8 --repeat 3     # Generates three 8-character strings
```

### Toss

Generates a random boolean (`50/50` chance)

```sh
random toss
```

#### Flags

The weights can be adjusted using the `--weight` flag. (defaults to `0.5`)

The `--coin` flag reformats the result as `Heads` or `Tails`

The `--repeat` flag allows you execute the command multiple times

#### Examples

```sh
random toss                         # Generates a random boolean
random toss --coin                  # Flip a coin
radom toss --coin --repeat 3        # Toss three coins
```

### Roll

Rolls die

```sh
random roll [DIE]
```

#### Arguments

Roll die `[DIE]` accepts input in the `{n}d{S}` format; where `{n}` is the number of dice thrown and `{S}` is the number of sides each die has. (example: `3d8` - Three 8-sided die)

defaults to `1d20`

#### Examples

```sh
random roll                     # Rolls a 1d20
random roll 3d8                 # Rolls 3d8
```

---

## License

> [MIT License](./LICENSE)