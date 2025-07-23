# `#[auto_*(*<*>)]` -> `#[auto_*(generics(*))]`

```regexp
/(auto_\w+)\(\w+<(.*?)>\)/\1(generics(\2))/
```