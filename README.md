# watchfile
The file watcher algorithm integrated into various bots, mainly used to track .osdf file changes.

## Language

Written in Rust, 100%. Note that this doesn't have a Cargo project just for the sake of simplicity. It doesn't have any dependencies, so you just need to compile it with `rustc` and you're good to go. Or create a Cargo project and add this file to it, doesn't matter.

## Usage

> [!WARNING]
> Is not meant, nor optimized, for general use. It is a very specific algorithm for a very specific use case.

### Example

You can find it in `sample.rs` file. Simply run it (make sure that `watchfile.rs` is in the same directory) and it will start watching the `smpl.txt` file. You can change the file content and see the changes being printed in the console.

## License

BSD-3-Clause (see [LICENSE](LICENSE) file).

## Author

[LyubomirT](https://github.com/LyubomirT) with The Orange Squad.

