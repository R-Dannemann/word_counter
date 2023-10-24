# Wortfrequenz-Zähler in Rust

Ein einfaches Rust-Programm zum Zählen der Häufigkeit von Wörtern in einer Textdatei.

## Verwendung

1. Stelle sicher, dass du Rust und Cargo installiert hast. Wenn nicht, [hier herunterladen](https://www.rust-lang.org/tools/install).

2. Lade das Repository herunter oder klone es:

```sh
git clone https://github.com/R-Dannemann/word_counter.git
```
---
1. Navigiere in das Verzeichnis des Projekts:

```sh
cd "Speicherort_des_Repositorys"
```

2. Kompiliere das Programm (Debug-Version):

```sh

cargo build
```
3. Führe das Programm aus und gib den Pfad zur Textdatei an:

```sh

cargo run Pfad/Zur/Textdatei.txt
```


Beispiel

Wenn du eine Textdatei namens beispiel.txt im selben Verzeichnis wie das Programm hast, kannst du es folgendermaßen ausführen:

```sh
cargo run beispiel.txt
```
Im Verzeichnis ist die engl. Version von Jules Verne - Euine Reise zum Mittelpunkt der Erde, um es gleich auszuprobieren.

```sh
cargo run a_journey_to_the_centre.txt
```

Das Programm wird die häufigsten 10 Wörter im Text anzeigen.

## Lizenz

MIT License

## Viel Spaß beim Ausprobieren
