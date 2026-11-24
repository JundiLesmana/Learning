# Learning Notes

Website dokumentasi catatan belajar dibangun dengan **Rust** menggunakan framework Axum.

## Fitur

- Tema **Liquid Glass** — frosted glass, blur, transparansi
- **Dark / Light mode** — tersimpan di localStorage
- Navbar floating, responsif
- Halaman: Beranda, About, MyLearn
- Sidebar MyLearn dengan navigasi topik
- Simulasi interaktif:
  - **Insertion Sort** — animasi step-by-step, input kustom, kecepatan adjustable
  - **Bubble Sort** — animasi perbandingan & pertukaran, input kustom

## Halaman

| Path | Deskripsi |
|------|-----------|
| `/` | Beranda |
| `/about` | Tentang |
| `/mylearn` | Daftar pembelajaran |
| `/Insert-Sorting` | Simulasi Insertion Sort |
| `/Bubble-Sorting` | Simulasi Bubble Sort |

## Menjalankan

```bash
# Pastikan Rust sudah terinstall
# Install: https://rustup.rs/

cd system
cargo run
# Server: http://localhost:3000
```

Atau gunakan script:

```bash
bash system/run.sh
```

## Teknologi

- **Rust** — bahasa pemrograman
- **Axum** — web framework async
- **Tokio** — async runtime
- **CSS** — Liquid Glass tema (pure CSS, tanpa framework)
- **JavaScript** — simulasi sorting (vanilla JS)
