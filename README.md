This is a small library that produces grid papers. The resulting paper either
contains actual grids or have dots at the vertices.

This is intended to be compiled into web assembly and used via a browser. A
live version can be found at
[https://dec41.user.srcf.net/includes/grid/](https://dec41.user.srcf.net/includes/grid/)

The only dependency (apart from wasm-bindgen) is flate2, which is used to
produce compressed pdf files. This dependency can be easily removed at the
expense of large file sizes. The difference is somewhat significant when
producing large dotted papers --- to produce an X by Y grid,
we need to draw X + Y lines, but to produce the corresponding dotted paper, we
need to draw XY dots.

# Compiling
Compiling requires `cargo` to be installed. Once it is installed, run
```
 $ ./build.sh
```
to compile. The compiled web assembly file will be written into `dist/`. The
directory `dist/` should then be served via a web server. Be sure to use a web
server that uses the correct MIME type for webassembly files, e.g. `emrun`. To
serve with Apache, add the following line to `.htaccess`.
```
AddType application/wasm .wasm
```

# Acknowledgements
The pdf part of the code is based on [rust-pdf](https://github.com/kaj/rust-pdf)
