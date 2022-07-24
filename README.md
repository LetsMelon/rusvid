# rusvid

After Effects alternative written in Rust ✨

(no gui or cli, under active development)

## How to use

Dependencies:
- rust + cargo nightly (because of `Rc::get_mut_unchecked`)
- ffmpeg

```sh
$ git clone https://github.com/LetsMelon/rusvid.git
$ cd rusvid
rusvid/ $ cargo run -r # Debug mode is really slow
```
