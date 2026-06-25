# Rust
https://rust-lang.org/

## Install
https://rust-lang.org/tools/install/

Checkup:
```shell
rustc --version
cargo --version
```

## Compilateur `rustc`

```shell
rustc main.rs
./main # ou main.exe
```

## Projet géré avec Cargo
Création d'un nouveau projet
```shell
cargo new basics
cargo new 02-Basics --name basics
```

ou dans le dossier courant:

```shell
cargo init
```

Executer en mode dev/debug
```shell
cargo run
```

Tests
```shell
cargo test
```

Build
```shell
cargo build
cargo build --release
```

## Ajout dépendances
```
cargo add chrono
cargo add --dev rstest
```
