# iw5-survival-tweaks

Do you recall requesting backup from ally delta or GIGN forces during a difficult wave, only to find that they were quickly defeated? With this modification, you can rectify that issue by granting godmode and/or no-target special flags to your troops, turrets, and other units.

Works with `iw5sp.exe` v1.9.461:

```
Size: 5.39 MB (5,653,800 bytes)
SHA-256: 1B90CF6C96C2E14C40F457378B486407DA822DB43C86A89D3E4A3072B4E89EB2
```

## Features

- Unbreakable riot shield (just like IW4SP).
- God mode, demigod mode, and no target for allies and turrets.

## Building

### Prerequisties
- Install [Rust](https://rustup.rs) if you haven't already.
- Add the x86 32-bit toolchain to your rustup's build targets:

```shell
$ rustup target add i686-pc-windows-msvc
```

### Getting the source code and compiling

- Navigate to a working directory of your choice and clone the repository:

```shell
$ git clone --recursive --depth=1 https://github.com/ifarbod/iw5-survival-tweaks
```

- Build the code:

```shell
$ cd iw5-survival-tweaks
$ cargo build --lib --target=i686-pc-windows-msvc
```

- Once it's done, install [Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader) to the root of your game directory, I found that `d3d9.dll` works well, however, you might want to use another DLL in case of using a graphics mod like ReShade.
- Create a `plugins` folder.
- Copy the resulting `.dll` file to that folder, rename the extension to `.asi`.
  - Alternatively, you can symlink it like this, if you want to devlop the mod further:
  ```
  mklink iw5_survival_tweaks.asi C:\$Files\Projects\Rust\iw5-survival-tweaks\target\i686-pc-windows-msvc\debug\iw5_survival_tweaks.dll
  ```
  
- After doing everything correctly, you should see a terminal screen attached to the game as you open it.
- 
