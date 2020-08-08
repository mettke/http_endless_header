# mean_image

Small attack script that create a png image file with the dimensions of 512x512 and a manipulated header which contains the dimension 65536x65536. Badly written image software may allocate a buffer based on the header alone resulting the program to run OOM.

## Execute

Start it using

```sh
cargo run --bin mean_image
```

## Results

There will be a file called `output.png` at the root of this repo. Use it with caution as opening the file might result in the program trying to allocate 12GB of RAM.
