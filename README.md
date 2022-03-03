libtorrent-rs
=============

Examples to use libtorrent-sys (cxx bindings)

Examples
========

1) version
----------
Print libtorrent version to the standard output.

```RUSTFLAGS="-C linker=g++" CXX=g++ cargo run --example version```

2) magnet2torrent
-----------------
Conversion magnet link => .torrent file.

```RUSTFLAGS="-C linker=g++" CXX=g++ cargo run --example magnet2torrent <magnet>```

