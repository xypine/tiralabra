# Testing

Unit tests are located at the bottom of each source file.

End to End tests (ensuring that the whole algorithm behaves as expected) reside in `src/wave_function_collapse/e2e_tests.rs`.

TBA: Detailed descriptions of tests

### Run all tests:

```bash
cargo test
```

### Generate a coverage report:

```bash
cargo tarpaulin -o html
```

then see `tarpaulin-report.html`
