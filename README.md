# SEBAS
SEBAS – Simply Elegant Bookmarked Alternatives for commandS

To design a CLI tool like SEBAS in Rust that meets your requirements, here’s a comprehensive approach:

---

### 1. **Core Design Goals**
- **GNU Compatibility**: Ensure the CLI adheres to GNU standards for consistency and familiarity.
- **Simplicity**: Keep commands concise and intuitive.
- **Flexibility**: Support nested `.sebas` folders with command inheritance.
- **Optional GUI**: Provide a simple GUI for users who prefer a visual interface.
- **JSON Storage**: Store commands in a structured JSON format for easy parsing and extensibility.

---

### 2. **Sub-Commands Interface Design**
Here’s a suggested structure for your CLI commands:

#### **Core Commands**
- **Add Command**: `sebas a <COMMAND> [--group GROUP] [--comment COMMENT]`
  - Adds a command to the nearest `.sebas` folder.
  - Example: `sebas a "git commit -m 'Initial commit'" --group "Git" --comment "Standard commit"`
- **List Commands**: `sebas ls [--group GROUP]`
  - Lists all commands (optionally filtered by group).
  - Example: `sebas ls --group "Git"`
- **Edit Command**: `sebas e <COMMAND_INDEX> [--new-command COMMAND] [--new-group GROUP] [--new-comment COMMENT]`
  - Edits an existing command.
  - Example: `sebas e 1 --new-command "git status" --new-comment "Check git status"`
- **Remove Command**: `sebas rm <COMMAND_INDEX>`
  - Removes a command by its index.
  - Example: `sebas rm 1`
- **Search Commands**: `sebas s <QUERY>`
  - Searches commands by keyword.
  - Example: `sebas s "commit"`

#### **Folder Management**
- **Init Folder**: `sebas init [PATH]`
  - Creates a `.sebas` folder at the specified path (defaults to current directory).
  - Example: `sebas init`
- **Sync Folders**: `sebas sync`
  - Syncs commands from nested `.sebas` folders to the home folder.
  - Example: `sebas sync`

---

### 3. **JSON Structure**
Here’s an example of how commands could be stored in JSON:

```json
{
  "commands": [
    {
      "command": "git commit -m 'Initial commit'",
      "group": "Git",
      "comment": "Standard commit",
    },
    {
      "command": "docker build -t my-image .",
      "group": "Docker",
      "comment": "Build Docker image",
    }
  ]
}
```

---

### 4. **Optional GUI**
An optional GUI could be built using a library like `egui` or `iced`. The GUI would:
- **List Commands**: Display all commands with their groups and comments.
- **Add/Edit Commands**: Provide a form for adding or editing commands.
- **Search**: Include a search bar to filter commands.
- **Sync**: Add a button to sync nested `.sebas` folders.

 Simple Indexing or Dynamic Resolution Options Choose between simple indices or hash-based identifiers:

Option 1: Dynamic, Context-Aware Indexes

    Assign numbers dynamically when a list is displayed. The numbers refer only to the current session of resolved commands.
        Easy to maintain and user-friendly.
        Indexes reset every time sebas ls is run.
        Example:

$ sebas ls
[1] docker build -t my-app . (./.sebas)
[2] git commit -m 'Initial commit' (~/project/.sebas)

After listing, users can edit/remove based on the transient index:

        $ sebas e 1 --new-command "docker build -t my-new-app ."

Option 2: Hash-Based Unique Identifiers

    Generate a unique hash (e.g., a short SHA256, UUID, or custom format based on the command text).
    Commands are always referred to by their hash, making them portable and consistent across CLI sessions.
        Example:

$ sebas ls
[a1f1] docker build -t my-app . (./.sebas)
[cff3] git commit -m 'Initial commit' (Root/.sebas)

Users then pass the hash when editing or removing commands:

$ sebas e a1f1 --new-command "docker build -t my-new-app ."
