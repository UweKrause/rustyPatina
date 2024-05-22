# rustyPatina

A rusty command line application to apply some patina to a photo.
Actually the patina effect is not yet implemented.

Actually currently there are no effects available yet.
See ToDo.

Actually this tool will just be a wrapper around the crate [image](https://docs.rs/image/latest/image/)
to use it from the command line.
So, to be clear, all kudos goes to the authors and contributors of [image](https://docs.rs/image/latest/image/).

ToDo:

- [ ] add command line interface to set input image and output image with parameter
- [ ] add command line interface for operations
- [ ] allow image input from unix pipe/stdin
- [ ] allow image output to unix pipe/stdout

## image operations

The goal
(since this is a wrapper around the rust crate [image](https://docs.rs/image/latest/image/))
is to support all operations that
[image](https://docs.rs/image/latest/image/)
supports.
