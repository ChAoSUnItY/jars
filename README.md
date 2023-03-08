# jars
`jars` is a simple utility library allows users to extract jar files on file system based on given
extraction rules.

## Usage

```rs
import jars::{jar, JarOptionBuilder};

let jar = jars::jar("sample/rt.jar", JarOptionBuilder::default())?;

for (file_path, content) in jar.files {
    // ...
}
```

## License
Copyright © 2023, [Kyle Lin (ChAoS-UnItY)](https://github.com/ChAoSUnItY).
Released under the [MIT License](LICENSE).
