<h1>Random</h1>

A command-line interface to do random stuff. Generate random strings and numbers, flip a coin, make decisions and resolve dilemmas. There is probably something in your life that can use a bit of randomness.

<details>
    <summary>Table of Contents</summary>

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
  - [💻 Toss](#-toss)
    - [Flags](#flags-3)
    - [Examples](#examples-3)
  - [💻 Roll](#-roll)
    - [Arguments](#arguments-2)
    - [Flags](#flags-4)
    - [Examples](#examples-4)
  - [💻 Shuffle](#-shuffle)
    - [Arguments](#arguments-3)
    - [Examples](#examples-5)
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

`--repeat, -r`: Number of times to repeat the execution of this command.

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

`--repeat, -r`: Number of times to repeat the execution of this command.

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

`--length, -l`: Specify the length of the generated strings (default: `16`).

`--charset, -c`: Specify the character-set to use. options: `lowercase`, `uppercase`, `alphabets`, `numbers`, `alphanumeric`, `special` and `all`.

`--repeat, -r`: Number of times to repeat the execution of this command.

#### Examples

```sh
random string                           # Generates a random 16-character string
random string --length 8                # Generates a radom 8-character string
random string --length 8 --repeat 3     # Generates three 8-character strings
```

### 💻 Toss

Generates a random boolean (50% chance to return `true` and `false`).

```sh
random toss
```

#### Flags

`--weight, -w`: Probabilistic weight of getting `true` (default: `0.5`)

`--coin, -c`: Reformats the result as `Heads` or `Tails` (instead of `true` or `false`)

`--repeat, -r`: Number of times to repeat the execution of this command.


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

Roll die `[DIE]` accepts input in the `{n}d{S}` format; where `{n}` is the number of dice thrown and `{S}` is the number of sides each die has. (example: `3d8` - Three 8-sided die)

default: `1d20`

#### Flags

`--with-advantage, -a`: Rolls the die with advantage. Takes the greatest value.

`--with-disadvantage, -d`: Rolls the die with disadvantage. Takes the smallest value.

#### Examples

```sh
random roll                         # Rolls a 1d20
random roll 3d8 --with-advantage    # Rolls 3d8 with advantage
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

#### Examples

```sh
random shuffle 1 2 3 4 5 6 7 8 9        # Shuffles the numbers 8 5 9 1 7 3 6 4 2
gh repo list | random shuffle           # Shuffles the repository list
```

---

## 📑 License

> [MIT License](./LICENSE)