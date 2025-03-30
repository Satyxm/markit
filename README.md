# markit

## Installation

### From Git

```bash
git clone https://github.com/Satyxm/markit.git
cd markit
cargo build --release
```

### Using Cargo (if available on crates.io)

```bash
cargo install markit
```

## Usage

`markit 1.0.0`  
Convert Markdown text to HTML.

### USAGE:

```bash
markit [OPTIONS]
```

### OPTIONS:

- `-h`, `--help`  
  Print help information

- `-V`, `--version`  
  Print version information

### EXAMPLES:

#### Convert a single line of Markdown to HTML:
```bash
$ markit
Enter Markdown text: # Hello, World!
This is a **Markdown** to HTML conversion example.
```

Output:
```html
<h1>Hello, World!</h1>
<p>This is a <strong>Markdown</strong> to HTML conversion example.</p>
```

#### Convert multiline Markdown to HTML:
```bash
$ markit
Enter Markdown text: 
# Hello, World!
This is some **Markdown** text.
It spans across multiple lines.

## Subheading
More content here.
```

Output:
```html
<h1>Hello, World!</h1>
<p>This is some <strong>Markdown</strong> text.
It spans across multiple lines.</p>
<h2>Subheading</h2>
<p>More content here.</p>
```

---

Any feature addition contributions are allowed. Just open an issue and make a pull request. remember to first fork the project then clone it and make changes on a new branch and then submit. 