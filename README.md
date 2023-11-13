# `random`

A command-line interface for when you need some randomness. Generate random strings and numbers, flip a coin, roll dice, or select one of many.

<details>
    <summary>Table of Contents</summary>

- [`random`](#random)
  - [📖 Usage](#-usage)
  - [🖋 Commands](#-commands)
    - [💻 Select](#-select)
      - [Arguments](#arguments)
      - [Flags](#flags)
      - [Examples](#examples)
    - [💻 Number](#-number)
      - [Arguments](#arguments-1)
      - [Flags](#flags-1)
      - [Examples](#examples-1)
    - [💻 String](#-string)
      - [Flags](#flags-2)
      - [Examples](#examples-2)
    - [💻 UUID](#-uuid)
      - [Examples](#examples-3)
    - [💻 Toss](#-toss)
      - [Flags](#flags-3)
      - [Examples](#examples-4)
    - [💻 Roll](#-roll)
      - [Arguments](#arguments-2)
      - [Flags](#flags-4)
      - [Examples](#examples-5)
    - [💻 Shuffle](#-shuffle)
      - [Arguments](#arguments-3)
      - [Flags](#flags-5)
      - [Examples](#examples-6)
  - [📑 License](#-license)

</details>

## 📖 Usage

```sh
random [COMMAND] [FLAGS]
```

Use the `help` command (or `--help, -h` flag) to see more details. `random help` or `random --help` or `random [COMMAND] --help`.

---

## 🖋 Commands

| Command               | Description                           |
| --------------------- | ------------------------------------- |
| [`select`](#select)   | Select one entry from a list randomly |
| [`number`](#number)   | Generate a random number              |
| [`string`](#string)   | Generate a random string              |
| [`uuid`](#uuid)       | Generate a random UUID/GUID           |
| [`toss`](#toss)       | Toss a coin                           |
| [`roll`](#roll)       | Roll a die                            |
| [`shuffle`](#shuffle) | Shuffles the given list               |

### 💻 Select

Randomly select one entry from a list.

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

- `--repeat, -r` or `--count, -c`: Number of times to repeat the execution of this command.

- `--separator, -s` or `--delimiter, -d`: Separator to use to format the results (default: `\n`)

```sh
gh repo list | random select --repeat 3
```

#### Examples

```sh
random select pizza burger pasta        # Selects either pizza, burger or pasta
gh repo list | random select            # Select one entry piped through stdin
```


### 💻 Number

Generate random numbers.

```sh
random number [NUM1] [NUM2]
```

#### Arguments

`[NUM1]` and `[NUM2]` are optional positional parameters that determine the range within which to generate the number.

- If both `[NUM1]` and `[NUM2]` are specified, the command generates a number between `[NUM1]` and `[NUM2]`.
- If only `[NUM1]` is specified, the command generates a number between `0` and `[NUM1]`.
- If no parameters are specified, the command generates a number between `0` and `1000`.

#### Flags

- `--repeat, -r` or `--count, -c`: Number of times to repeat the execution of this command.

- `--separator, -s` or `--delimiter, -d`: Separator to use to format the results (default: `\n`)

#### Examples

```sh
random number                   # Generates a number between 0 and 1000
random number 5                 # Generates a number between 0 and 5
random number 5 15              # Generates a number between 5 and 15
random number 5 10 --repeat 5   # Generate 5 numbers between 5 and 15     
```

### 💻 String

Generate random strings using alphanumeric and special characters.

```sh
random string
```

#### Flags

- `--length, -l`: Specify the length of the generated strings (default: `16`).

- `--charset`: Specify the character-set to use. options: `lowercase`, `uppercase`, `alphabets`, `numbers`, `alphanumeric`, `special` and `all`. If the charset is anything other than the specified options, the literal characters itself are used as the charset.

- `--repeat, -r` or `--count, -c`: Number of times to repeat the execution of this command.

- `--separator, -s` or `--delimiter, -d`: Separator to use to format the results (default: `\n`)

#### Examples

```sh
random string                           # Generates a random 16-character string
random string --length 8                # Generates a radom 8-character string
random string --length 8 --repeat 3     # Generates three 8-character strings
```

### 💻 UUID

Generate a random UUID/GUID

```sh
random uuid
# or
random guid
```

#### Examples

```sh
random uuid   # Generates a random uuid
random guid   # Does the same thing. guid is an alias for uuid
```

### 💻 Toss

Generates a random boolean (50% chance to return `true` and `false`).

```sh
random toss
```

#### Flags

- `--weight, -w`: Probabilistic weight of getting `true` (default: `0.5`)

- `--coin`: Reformats the result as `Heads` or `Tails` (instead of `true` or `false`)

- `--repeat, -r` or `--count, -c`: Number of times to repeat the execution of this command.

- `--separator, -s` or `--delimiter, -d`: Separator to use to format the results (default: `\n`)

#### Examples

```sh
random toss                         # Generates a random boolean
random toss --coin                  # Flip a coin
random toss --coin --repeat 3       # Toss three coins
```

### 💻 Roll

Roll a die.

```sh
random roll [DIE]
```

#### Arguments

Roll die `[DIE]` accepts input in the `{n}d{S}+X` format; where `{n}` is the number of dice thrown and `{S}` is the number of sides each die has and `{X}` is the number to add to the result. (example: `3d8` - Three 8-sided die, 2d6+4 - Two 6-sided dice and add 4 to the result)

default: `1d20`

#### Flags

- `--with-advantage, -a`: Rolls the die with advantage. Takes the greatest value.

- `--with-disadvantage, -d`: Rolls the die with disadvantage. Takes the smallest value.

- `--difficulty-check`: Difficulty check. The rolls total must be higher than this value to succeed.

#### Examples

```sh
random roll                         # Rolls a 1d20
random roll 3d8 --with-advantage    # Rolls 3d8 with advantage
random roll 1d8+5                   # Rolls a 1d8 and adds 5
random roll 1d8+2d6+3               # Rolls 1d8 and 2d6 and adds 3 to the results
random roll 2d8 --difficulty-check 10 # Roll 2d8 and check against 10 
```

### 💻 Shuffle

Shuffles the given list.

```sh
random shuffle [...ENTRIES]
```

#### Arguments

Takes a variable amount of arguments representing the list of all entries.

```sh
random shuffle one two three four five ...
```

Can also accept input from a redirected stdin.

```sh
gh repo list | random shuffle
```

#### Flags

- `--separator, -s` or `--delimiter, -d`: Separator to use to format the results (default: `\n`)

#### Examples

```sh
random shuffle 1 2 3 4 5 6 7 8 9        # Shuffles the numbers 8 5 9 1 7 3 6 4 2
gh repo list | random shuffle           # Shuffles the repository list
```

---

## 📑 License

This project is licensed under the [MIT License](./LICENSE).
