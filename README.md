# Search File

This is a grep like tool for searching lines which contains a given keyword in a file.

## Installation

<details>
<summary>Using Prebuilt Binaries</summary>

- <details>
  <summary>With Bash</summary>

  ```sh
  curl -fsSL "https://bina.egoist.dev/imranbarbhuiya/search_file" | sh
  ```

  > Using [Bina](https://bina.egoist.dev/)

  </details>

- <details>
  <summary>Manual Installation</summary>

  Prebuilt binaries are available for Windows, Linux, and macOS and can be found
  attached to the [latest release](https://github.com/imranbarbhuiya/search_file/releases/latest).

  </details>

</details>
<details>
<summary>Building From Source</summary>

Install [Rust](https://www.rust-lang.org/tools/install) and then run:

```sh
cargo install search_file
```

</details>

<br />

## Usage

```sh
# Case sensitive match
search_file <keyword> <file>
# Case insensitive match
IGNORE_CASE=1 search_file <keyword> <file>
```

</br>

This is my first rust project. So I'm just learning how to create a rust project and publish it to crate.io.
