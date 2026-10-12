# DevDock

**DevDock** is a lightweight, cross-platform development workspace manager written in Rust.

Launch your entire development environment with a single command:

```bash
dock TestProject
```

Open VS Code projects, folders, terminals, websites, and custom commands from a single session.

**Version:** `v0.5.1` | **Platforms:** Windows & Linux | **License:** MIT

## Features

- One-command workspace launching
- Graphical Session Editor with dark UI
- VS Code, folders, terminals, websites, and custom commands
- Interactive folder selection and session management
- TOML-based session configuration
- Dry-run mode
- Windows and Linux support

## Installation

Download the latest package from [GitHub Releases](https://github.com/MajkiiWasTaken/DevDock/releases).

### Windows

Extract `DevDock-windows-x86_64.zip` and run:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -BinaryPath .\dock.exe
```

Open a new terminal and verify:

```powershell
dock --version
```

### Linux

Extract `DevDock-linux-x86_64.tar.gz` and run:

```bash
bash ./install.sh ./dock
```

Ensure `~/.local/bin` is in your `PATH`.

### Build from source

```bash
git clone https://github.com/MajkiiWasTaken/DevDock.git
cd DevDock
cargo build --release
```

## Usage

| Command | Description |
|---|---|
| `dock setup` | Open GUI to create a session |
| `dock setup NAME` | Edit an existing session in GUI |
| `dock NAME` | Launch a session |
| `dock list` | List configured sessions |
| `dock add NAME --path PATH` | Create a session using CLI |
| `dock info NAME` | Display session information |
| `dock edit NAME` | Edit session TOML manually |
| `dock remove NAME` | Remove a session |
| `dock NAME --dry-run` | Preview session actions |

## Graphical Session Editor

Create a new workspace using the built-in graphical editor:

```powershell
dock setup
```

The editor provides a sidebar with six sections:

- **General** — session name and main directory
- **VS Code** — projects to open
- **Folders** — additional file manager windows
- **Terminals** — working directories and shells
- **Websites** — URLs to open
- **Commands** — custom executable commands and arguments

Select directories through the system file picker and save your configuration without editing TOML manually.

To edit an existing workspace:

```powershell
dock setup TestProject
```

## Example

Create and configure your development workspace:

```powershell
dock setup
```

Launch it later with:

```powershell
dock TestProject
```

Preview the actions without launching anything:

```powershell
dock TestProject --dry-run
```

## Configuration

Sessions are stored as individual TOML files.

**Windows:** `%APPDATA%\DevDock\sessions\`

**Linux:** `~/.config/devdock/sessions/`

Example:

```toml
name = "TestProject"
folder = 'C:\Projects\TestProject'
vscode = ['C:\Projects\TestProject\Firmware']
folders = ['C:\Projects\TestProject\Documentation']
websites = ["https://github.com"]

[[terminals]]
path = 'C:\Projects\TestProject\Firmware'
shell = "powershell"

[[commands]]
program = "git"
args = ["status"]
cwd = 'C:\Projects\TestProject'
```

Sessions are preserved during updates and uninstallation.

## Development

```bash
cargo fmt --check
cargo check
cargo test
cargo build --release
```

## Author

**Michal Švrček**

## License

Distributed under the MIT License. See [LICENSE](LICENSE).