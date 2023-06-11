# Working with the Core `thcon` Binary
This project uses a pretty standard Go build pattern:

```sh
# Compile
go build

# Test
go test

# Lint
# Install staticcheck: https://staticcheck.io/docs/getting-started/
staticcheck
```

Unfortunately, cross-compilation doesn't work very well due to the native
dependencies involved.

# Building documentation
1. [Install pnpm](https://pnpm.io/installation)
2. `pnpm -C docs/ start`
3. Open `https://localhost:8080` in your browser
