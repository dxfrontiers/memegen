# Memegen-GTK
A meme genrator written in Rust

A tiny utility to showcase the use of GTK+ 3 and rust on windows and linux.

To generate a meme follow these steps:
- Think of a funny image macro
- Open the corresponding image via the open button
- Enter your text in the textbox on the right,  an empty line creates a new, freely movable text field
- Move the text areas to the desired position by clicking on them and moving the mouse
- You can scroll while placing the mouse over text to change its font size
- Save the image by clicking on the save button
- Enjoy!


## Building
Builds are run on (arch-)linux and may work on windows. Executables are crosscompiled to work on windows.

### For Linux
- Ensure that gtk3 is installed in your system. How to achieve this depends on your distribution and you will most likely find a tutorial on how to do this.
- go to the memegen-gtk directoyr `memegen-gtk`
- Invoke cargo with `cargo build --release`

### For Windows (on Linux)
- Refer to [this tutorial](https://gtk-rs.org/docs-src/tutorial/cross) to set up your environment.
- Ensure you have set the `GTK_INSTALL_PATH` variable in the used shell: e.g. `export GTK_INSTALL_PATH=/usr/x86_64-w64-mingw32`.
- You might want to apply [this workaround](https://github.com/rust-lang/rust/issues/48272#issuecomment-429596397), depending on your version, if you receive an error complainign about `__onexitbegin`
- Invoke cargo with `cargo build --release --target=x86_64-pc-windows-gnu`
- run `populate_release.sh` to package all dependencies into a zip that can be distributed to windows machines 



## Modify the UI
The UI is based on a glade-file, that can be edited with glade:

![icon](doc/editor.jpg)

You can move compentnts around without touching the code, as long as you don't change the IDs.



