# Yatap - Yet Another Terminal App-Selector

or just `ta`, is a TUI application that lets you add folders to search for apps that you want to open in a Tmux session for example

this crate is in alpha

## Usage

Running `ta` will search for the folders you added in your config and fuzzy find on them

Use `ta --help`

```
Usage: ta [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to config file
  -h, --help             Print help
  -V, --version          Print version
```

### Configuring 

```
paths: [] // Paths that you want to select folders
```

Example: 

```
paths: ["/home/oacs/dev", "/home/oac/.config"] 
```


## Installation

### Cargo

Install with `cargo install yatap` or

### From source

Clone the repository and install using ```cargo install --path . --force```

## See also

[tmux-sessionizer](https://crates.io/crates/tmux-sessionizer)

## Draft Idea

![TA - Design](https://user-images.githubusercontent.com/13282482/221361770-c0dbb24d-9bff-4a60-ba93-a81d845c4dee.png)
