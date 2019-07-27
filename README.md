# Welcome to jenr!

## What is this?

`jenr` is a Jenkins CLI client built in Rust, intended to be used to save you the effort to go through crappy web interfaces or web interfaces at all for that matter.


## How can I use it?

`jenr` is still in early development so it's not installable through Cargo yet.

You'd need to:
  1. Clone the repo
  2. Build it: `cargo build`
  3. Run it: `cargo run`

`jenr` is able to access your Jenkins instance by looking at the `JENKINS_HOME` env var so make sure to set that before using it!


--------------------------------------------------------------------
**If you don't have `cargo` installed just yet, you can [do it now](https://doc.rust-lang.org/cargo/getting-started/installation.html)!**
