# Rust Bindgen example

I wrote a simple example to use Bindgen with a few more files and folders such that there is an example in the wild using `clang_args()`

## Building + Run
- `cargo build`
- `cargo run`


Im uploading a prebuilt library. But you can compile for your own platform it you want.

You can compile the library mybind manually from your terminal using and running this from the root directory of the project

```
cd mybind \
&& gcc -Wall\
-I./include -I./include2\
-c test.c ./include/test2.c ./include2/test3.c \
&& ar -cvq libmybind.a test.o test2.o test3.o \
&& rm *.o \
&& cd ..
```
