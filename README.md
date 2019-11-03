This is a frontend to qrcode-rust

Options include:

- Text to encode
- Path to save image
- Approximate size of image
- Dark color to use
- Light color to use
- Disable border space

![](https://i.imgur.com/LrDnIzK.jpg)

Colors can be changed, though it might cause problems when scanning:

`qool "this is some text to encode" --dark-color purple --light-color pink`

![](https://i.imgur.com/mdbOyiA.jpg)

If no path is provided it will save images in `~/qool-codes/123.png` using the unix seconds for file names.