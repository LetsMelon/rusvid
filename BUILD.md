# Introduction

This document describes how to build the application and platform binaries.

# Requirements

Download and install the following software packages:

* [Rust nightly](https://www.rust-lang.org/tools/install)
* [Git 2.38.1](https://git-scm.com/downloads)
* [FFmpeg 5](https://ffmpeg.org/download.html) 

    ```sh
    apt-get install -y ffmpeg libavutil-dev libavformat-dev libswscale-dev
    ```

## Repository

Clone the repository as follows:

```sh
git clone https://github.com/LetsMelon/rusvid
```

The repository is cloned.

# Build

Packages:

- `rusvid_cli` (binary)
- `rusvid_core`  (library)
- `rusvid_effect` (library)
- `rusvid_lib` (library)
- `rusvid_server` (binary)
- `rusvid_video_encoder` (library)
- `rusvid_wasm` (wasm)

Build a package from rusvid as follows:

```sh
cd rusvid
cargo build -r -p $NAME
```

or

```sh
cd rusvid
cargo build -r --all
```

The applications and libraries are built.

# Run

After the application is compiled, run it using `./target/release/$NAME`.

# Integrated development environments

This section describes setup instructions to import and run the application
using an integrated development environment (IDE). Running the application
should trigger a build.
