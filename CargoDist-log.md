Installation of cargo-dist: `cargo binstall cargo-dist`

Setup with defaults: `dist init --yes`

Local build (for inspection of generated files): `dist build`
```
Finished `dist` profile [optimized] target(s) in 41.13s
announcing v0.16.0
  koto-ls 0.16.0
    /Users/robertosaccon/GitHub/forks/koto-ls/target/distrib/source.tar.gz
      [checksum] /Users/robertosaccon/GitHub/forks/koto-ls/target/distrib/source.tar.gz.sha256
    /Users/robertosaccon/GitHub/forks/koto-ls/target/distrib/sha256.sum
    /Users/robertosaccon/GitHub/forks/koto-ls/target/distrib/koto-ls-aarch64-apple-darwin.tar.xz
      [bin] koto-ls
      [misc] LICENSE, README.md
      [checksum] /Users/robertosaccon/GitHub/forks/koto-ls/target/distrib/koto-ls-aarch64-apple-darwin.tar.xz.sha256
(base) robertosaccon@Robertos-Laptop koto-ls %
```

CI simulation: `dist plan`
```
(base) robertosaccon@Robertos-Laptop koto-ls % dist plan
announcing v0.16.0
  koto-ls 0.16.0
    source.tar.gz
      [checksum] source.tar.gz.sha256
    sha256.sum
    koto-ls-aarch64-apple-darwin.tar.xz
      [bin] koto-ls
      [misc] LICENSE, README.md
      [checksum] koto-ls-aarch64-apple-darwin.tar.xz.sha256
    koto-ls-aarch64-unknown-linux-gnu.tar.xz
      [bin] koto-ls
      [misc] LICENSE, README.md
      [checksum] koto-ls-aarch64-unknown-linux-gnu.tar.xz.sha256
    koto-ls-x86_64-apple-darwin.tar.xz
      [bin] koto-ls
      [misc] LICENSE, README.md
      [checksum] koto-ls-x86_64-apple-darwin.tar.xz.sha256
    koto-ls-x86_64-pc-windows-msvc.zip
      [bin] koto-ls.exe
      [misc] LICENSE, README.md
      [checksum] koto-ls-x86_64-pc-windows-msvc.zip.sha256
    koto-ls-x86_64-unknown-linux-gnu.tar.xz
      [bin] koto-ls
      [misc] LICENSE, README.md
      [checksum] koto-ls-x86_64-unknown-linux-gnu.tar.xz.sha256
```

THEN publishing release manually via Github website

=> workflow action failed at: `host` section of `release.yml` (but artifacts all got built)
