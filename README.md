# Welcome to vanity-text!

vanity-text is a program that prints out a given text file to the terminal. But slowly! 

Yeah its really just a ricing thing, no practical purpose, just for screenshots.

This is a relatively small practice project I created while learning rust, so I wouldn't expect the code to be all that great.

## Usage

```
vanity-text <path-to-file>
```

## Known issues & Todo

<pre>
Issues:
  Panicks if given a non utf-8 encoded file
  Doesn't properly reset the cursor positon when clearing the terminal

Todo:
  Add support for reading more than just utf-8 file types
    - Also includes adding binary and hexadecimal output modes!
  Potentially refactor the entire project
</pre>

## Building

```
git clone https://github.com/Chloezu/vanity-text.git
cd vanity-text
cargo update && cargo build
```

## Contributing

If you want to contribute to this project for some reason, then feel free to open up a pull request!

Im still new to rust so expect the code to be messy and probably poorly formatted