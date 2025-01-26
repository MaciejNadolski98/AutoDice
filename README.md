# Prerequisites

## Install Rust

### rustup

Download and run `rustup-installer.exe` from `https://rustup.rs/

In the command line, choose `1) Quick Install via the Visual Studio Community Installer`

Click `Skip this for now` in `Sign in to Visual Studio`.

Choose your favourite theme (should be Dark 😉) and Start Visual Studio.

`Continue without code` and close the Visual Studio window.

Go back to the Terminal and press enter to `Proceed with standard installation`.

Press Enter to continue.

### cargo and rustc

Enter Terminal (for instance `Developer Powershell for VS 2022`)

In the terminal, enter:

```
rustup default stable-msvc
```

## VS Code

Go to `https://code.visualstudio.com/`, Download for Windows and then run the installer.

Use default options in the installer and click through to the end.

## Install git and Git Bash

Go to `https://git-scm.com/download/win` and install git.

Check off `Windows Explorer Integration` and press `Next` until you're at `Choosing the default editor used by Git`.

There, choose `Use the Nano editor by default` and click Next through all remaining choices, click Finish.

Go to VS Code (restart if was running), press `Ctrl + Shift + P` and type `Select Default Profile`. There select `Git Bash`.

Press `Ctrl + P` to open terminal. Make sure it's `bash` (in the top-right of the terminal). If not, press the `+` next to the terminal name and delete the old terminal. New one should be `bash`.

## Download repository

Enter location where you want to have the project in the terminal and enter:

```
git clone https://github.com/MaciejNadolski98/AutoDice.git
```

Sign in to github using your credentials.

```
cd AutoDice
```

## Install Vulkan driver

Installation may depend on your architecture.

## Build and run the project

```
cargo run
```

# Extra

I recommend the following VS Code extensions

- rust-analyzer
- Rust Syntax
- GitLens

# Usage

In order to install dependencies, build the program and run it:

```
cargo run
```
