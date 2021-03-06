This is a frontend to [qrcode-rust](https://github.com/kennytm/qrcode-rust) that allows you to generate QR Code images and save them to a file.

# Options

- Text to encode
- Path to save image
- Approximate size of image
- Dark color to use
- Light color to use
- Disable border space

# Usage

Basic use is: `qool "something to encode"`

Quotation marks are necessary for text that contains spaces.

With a path: `qool mySite /home/me/pics/qrimg.png`

If no path is provided it will save images in `~/qool-codes/123.png` using the unix seconds for file names.

![](https://i.imgur.com/LrDnIzKl.jpg)

Colors can be changed, though it might cause problems when scanning:

`qool "this is some text to encode" --dark-color purple --light-color pink`

![](https://i.imgur.com/mdbOyiAl.jpg)

# More Examples

`qool cats ~/qr/cat.jpg --no-border`

`qool squirrels /home/me/pics/qr/sq.png --size 900 --dark-color grey`

# Print

The code can be printed to the terminal instead.

![](https://i.imgur.com/gZjmn1Rl.jpg)

# Installation

The Rust compiler is required.

Download zip and execute `install.sh`

This will place an executable in /bin/qool

# Help

run `qool --help` to check all available arguments.