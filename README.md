# 🌠OS Template

<p>🔧This is a template that can help you quickly build <code>your own OS</code>> using <code>rust</code>.</p>
<p>🦀<code>HanabiOS</code> is built on it.</p>

# 📕Prepare for it

1. <p>🌙Use the following command to install the <code>nightly version of rust</code>,</p><p>😀because we may need to use some features that only this version provides.</p>

```shell
rustup install nightly
```

2. <p>🔧Then set the toolchain used in this project to <code>nightly</code> version of rust.</p><p>🔭In the future we can enable some experimental features by adding feature flags at the beginning of <code>main.rs</code>,</p><p>😫such as adding <code>#![feature(asm)]</code> to enable experimental introverted compilation.</p>

```shell
rustup override add nightly
```

3. <p>🔧Configure your <code>target.json</code>,</p><p>📜here we only provide <code>target.json</code> to help you build the system for the <code>x86_64</code> architecture platform.</p><p>😀If you have other written <code>target.json</code>,</p><p>welcome to submit it to us!</p>

4. <p>📦Install xbuild and bootimage,</p><p>🏭they can help you quickly build an operating system image.</p>

```shell
cargo install cargo-xbuild
rustup component add llvm-tools-preview
cargo install bootimage
```

5. <p>✍From now on you can directly start writing the code of the operating system!</p>

# 🏃‍Build And Run

```shell
cargo bootimage --target target.json
qemu-system-x86_64 -drive format=raw,file=./target/target/debug/bootimage-hanabi.bin
```

# 🛑Precautions

1. <p>😫You will not be able to use anything in <code>std::</code>, </p><p>🔨you need to construct your own api and call it.</p>
2. <p>📜The <code>target.json</code> given here can only help you build an operating system for the <code>x86_64 platform</code>,</p><p>✍please write your own for other platforms!</p><p>🤝If you have <code>target.json</code> for other platforms,</p><p>🤣please submit it to us!</p><p>❤Thank you!</p>