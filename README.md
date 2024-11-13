# KJSPKG

![Crates.io Version](https://img.shields.io/crates/v/kjspkg?style=for-the-badge)
![Deployment Status](https://img.shields.io/github/actions/workflow/status/RedstoneWizard08/kjspkg/deploy.yml?branch=main&style=for-the-badge&label=Deployment)
![CLI Build Status](https://img.shields.io/github/actions/workflow/status/RedstoneWizard08/kjspkg/cli-build.yml?branch=main&style=for-the-badge&label=CLI%20Build)
![Lines of Code](https://tokei.rs/b1/github/RedstoneWizard08/kjspkg?style=for-the-badge&label=Lines+Of+Code)

A package manager & registry for KubeJS.

## CLI Installation

<details open>
<summary>Via script (Linux/MacOS only)</summary>
You can install the KJSPKG CLI with a handy script! Just run:

```sh
curl -fsSL https://github.com/RedstoneWizard08/kjspkg/raw/main/install_cli.sh | bash
```

Once you've run that, if `~/.local/bin` is in your `PATH`, it should be installed!
</details>

<details open>
<summary>Via PowerShell script (Windows only)</summary>
Coming soon!
</details>

<details>
<summary>With `cargo-binstall`</summary>
To install the KJSPKG CLI with `cargo-binstall`, you need to run:

```sh
cargo binstall kjspkg
```

As long as `~/.cargo/bin` is in your `PATH`, the CLI (`kjspkg`) should be available!
</details>

<details>
<summary>With `cargo install`</summary>
You can also install the KJSPKG CLI using `cargo install`, compiling it from the latest version on [crates.io](https://crates.io/crates/kjspkg)! Just run:

```sh
cargo install kjspkg
```

As long as `~/.cargo/bin` is in your `PATH`, the CLI (`kjspkg`) should be available!
Note that this option requires Rust to be installed: https://rustup.rs/
</details>

<details>
<summary>From source</summary>
If you want the most bleeding-edge unreleased version of the CLI, you can install it from source:

```sh
# For Linux/MacOS
git clone https://github.com/RedstoneWizard08/kjspkg
cd kjspkg
cargo build --release -p kjspkg
cp target/release/kjspkg /path/to/your/bin/dir/kjspkg
```

```pwsh
# For Windows
git clone https://github.com/RedstoneWizard08/kjspkg
cd kjspkg
cargo build --release -p kjspkg
cp target/release/kjspkg.exe C:\path\to\your\bin\dir\kjspkg.exe
```

Note that this requires `git`, rust, and for Windows, MSVC to be installed.
</details>

