# DevDock

**DevDock** is a lightweight, cross-platform command-line tool written in Rust for launching and managing development workspaces.

Open your entire development environment with a single command.

```bash
dock TestProject
```

DevDock can launch VS Code projects, file manager windows, terminals, websites, and custom commands from a single session configuration.

## Features

- Multiple VS Code projects per session
- File manager integration
- Terminal launching
- Website launching
- Custom commands
- TOML-based configuration
- Session management commands
- Dry-run mode
- Windows and Linux support
- No administrator privileges required for installation

## Installation

Download the latest package from [GitHub Releases](https://github.com/MajkiiWasTaken/DevDock/releases).

### Windows

Extract `DevDock-windows-x86_64.zip`.

Open PowerShell inside the extracted directory:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -BinaryPath .\dock.exe
```

Open a new terminal and verify:

```powershell
dock --version
```

### Linux

Extract `DevDock-linux-x86_64.tar.gz`:

```bash
mkdir devdock
tar -xzf DevDock-linux-x86_64.tar.gz -C devdock
cd devdock
bash ./install.sh ./dock
```

Reload your shell environment if necessary.

```bash
dock --version
```

### Build from source

Requirements:

- Rust toolchain
- Cargo

Clone the repository:

```bash
git clone https://github.com/MajkiiWasTaken/DevDock.git
cd DevDock
cargo build --release
```

On Windows:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

On Linux:

```bash
bash ./scripts/install.sh
```

## Usage

| Command | Description |
|---|---|
| `dock list` | List configured sessions |
| `dock add NAME --path PATH` | Create a session |
| `dock NAME` | Launch a session |
| `dock open NAME` | Launch a session explicitly |
| `dock info NAME` | Display session information |
| `dock edit NAME` | Edit the session TOML file |
| `dock remove NAME` | Remove a session after confirmation |
| `dock remove NAME --yes` | Remove without confirmation |
| `dock NAME --dry-run` | Preview session actions |
| `dock --version` | Show the installed version |

### Example

Create a session:

```powershell
dock add TestProject --path "C:\Projects\TestProject"
```

Edit the session:

```powershell
dock edit TestProject
```

Launch the session:

```powershell
dock TestProject
```

## Session configuration

DevDock uses TOML files to define workspaces.

### Example

```toml
name = "TestProject"
folder = 'C:\Projects\TestProject'

vscode = [
    'C:\Projects\TestProject\Firmware',
    'C:\Projects\TestProject\Manager'
]

folders = [
    'C:\Projects\TestProject\Documentation'
]

websites = [
    "https://github.com"
]

[[terminals]]
path = 'C:\Projects\TestProject\Firmware'
shell = "powershell"

[[commands]]
program = "git"
args = ["status"]
cwd = 'C:\Projects\TestProject\Firmware'
```

Custom commands launch separate processes. Their completion and exit status are not currently monitored by DevDock.

### Configuration locations

Windows:

```text
%APPDATA%\DevDock\sessions\
```

Linux:

```text
~/.config/devdock/sessions/
```

On Linux, `XDG_CONFIG_HOME` is respected if set.

Each session has its own file, for example:

```text
TestProject.toml
AnotherProject.toml
devdock.toml
```

Sessions are preserved when updating or uninstalling DevDock.

## Dry-run mode

Preview a session without launching applications:

```bash
dock TestProject --dry-run
```

This displays the configured workspace entries.

## Uninstallation

### Windows

Run the uninstaller from the extracted package or project:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

If using an extracted release package, run `.\uninstall.ps1` instead.

### Linux

```bash
bash ./scripts/uninstall.sh
```

If using an extracted release package, run `./uninstall.sh` instead.

Uninstalling removes the installed executable but preserves session configuration files.

## Development

Format:

```bash
cargo fmt
```

Check:

```bash
cargo check
```

Test:

```bash
cargo test
```

Build:

```bash
cargo build --release
```

## Roadmap

- Improved interactive session management
- More reliable terminal launching
- Session process tracking
- Duplicate application detection
- Advanced workspace layouts
- Additional operating system support

## Author

**Michal Švrček**

GitHub: [MajkiiWasTaken](https://github.com/MajkiiWasTaken)

## License

Distributed under the MIT License. See [LICENSE](LICENSE).