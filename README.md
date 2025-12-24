# Stencil

Stencil is a command-line tool for generating files from reusable templates. It helps you quickly scaffold new components, modules, configuration files, or any other repetitive file structure in your projects.

## How It Works

You define a "stencil" in a `.toml` file, which includes the file content, placeholder keys, and the desired output file extension. You can then add this stencil to the tool's registry and use it anytime to generate new files by providing values for the keys.

## Features

-   **Reusable Templates**: Create and save templates for any file type.
-   **Dynamic Content**: Use placeholders in your templates that get replaced with command-line arguments.
-   **Simple Configuration**: Templates are defined in easy-to-read TOML files.
-   **Centralized Management**: Stencils are stored in a central configuration directory, making them accessible from anywhere on your system.

## Building from Source

To build Stencil, you'll need Rust and Cargo installed on your system.

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/crustypub/stencil.git
    ```

2.  **Navigate to the project directory:**
    ```sh
    cd stencil
    ```

3.  **Build the project:**
    ```sh
    cargo build --release
    ```

The executable will be located at `target/release/stencil`. You can copy this to a directory in your system's `PATH` (e.g., `/usr/local/bin`) for easy access.

## Usage

Stencil operates in two modes: adding a new template (`-a`) and generating a file from a template (`-g`).

### 1. Creating a Stencil Template

First, create a `.toml` file to define your stencil. This file has three required fields:

-   `file_type`: The extension for the generated file (e.g., `".jsx"`).
-   `keys`: A list of strings that will be used as placeholders in the template.
-   `stencil`: A multi-line string containing the template content. Use `[[key]]` for placeholders.

Here is an example `react-component.toml` for a simple React component:

```toml
// This file is an example of the structure you should use.

file_type = ".jsx"

keys = ["componentName", "title"]

stencil = """
import React, { useState, useEffect } from 'react';

function [[componentName]]() {
  const [value, setValue] = useState(null);

  return (
    <div className="counter">
      <span className="title">[[title]]</span>
    </div>
  );
}

export default [[componentName]];
"""
```

### 2. Adding a Stencil to the Registry

Use the `-a` flag to add your `.toml` file to the Stencil registry. The name of the stencil will be the filename without the `.toml` extension.

```sh
stencil -a /path/to/your/react-component.toml
```

After running this command, the stencil is registered under the name `react-component` and can be used for generation.

### 3. Generating a File

Use the `-g` flag, followed by the stencil name and key-value pairs, to generate a file.

```sh
stencil -g <stencil_name> key1=value1 key2=value2
```

Using the `react-component` stencil we added earlier:

```sh
stencil -g react-component componentName=Header title="Welcome"
```

This will create a new file in the current directory with a random name (e.g., `aB1c.jsx`) containing the following content:

```jsx
import React, { useState, useEffect } from 'react';

function Header() {
  const [value, setValue] = useState(null);

  return (
    <div className="counter">
      <span className="title">Welcome</span>
    </div>
  );
}

export default Header;
```

#### Specifying an Output Directory

You can also specify an output directory as the last argument:

```sh
stencil -g react-component componentName=Footer title="Copyright" ./src/components
```

This will generate the file inside the `./src/components` directory.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
