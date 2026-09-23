# Roadmap 

> Panduan belajar Python terstruktur dari **nol** sampai **level expert**,
> lengkap dengan materi, latihan, dan proyek kecil untuk setiap tahap.
> Semua file latihan ditempatkan di folder `python/src/` mengikuti
> folder topik yang sudah ada (`function/`, `oop/`, dst).

---

## Cara Menggunakan Roadmap Ini

- Setiap **fase** terdiri dari beberapa **minggu**.
- Buat file latihan per minggu dengan pola penamaan:
  `python/src/<topik>/w<minggu>-<nomor>.py`

  - Contoh: `python/src/fundamental/w1-1.py`, `python/src/function/w7-3.py`

- **Satu topik = satu studi** sebelum lanjut. Jangan tebar-tebar.
- Jalankan file latihan dengan:
  ```bash
  python3 python/src/fundamental/w1-1.py
  ```
- Catat jawaban uji pemahaman di comment (`#`) atau file `catatan.md` per fase.
- Target realistis: **±20–30 menit/hari** atau **2–4 jam/minggu**.

---

## Peta Perjalanan

| Fase | Level | Estimasi Waktu | Hasil Akhir |
|------|-------|----------------|-------------|
| 0 | Persiapan | 3–5 hari | Environment siap & paham terminal |
| 1 | Fundamental | Minggu 1–4 | Bisa bikin program CLI sederhana |
| 2 | Struktur Data & Fungsi | Minggu 5–8 | Bisa olah data & menulis fungsi modular |
| 3 | Pemrograman Berorientasi Objek (OOP) | Minggu 9–12 | Bisa mendesain class & program terstruktur |
| 4 | Intermediate | Minggu 13–16 | Bisa baca/tulis file, testing, pakai library |
| 5 | Advanced | Minggu 17–20 | Bisa concurrency, async, pola desain |
| 6 | Expert | Minggu 21–24+ | Bisa optimasi, packaging, berkontribusi OSS |

---
## Fase 0 — Persiapan & Setup (3–5 hari)

**Tujuan:** install Python, dan menjalankan script pertama.

| Hari | Materi | Latihan |
|------|--------|---------|
| 1 | Apa itu Python, interpreter vs compiler, install Python (pyenv/apt/installer resmi) | `setup/w0-1.py` → print `sys.version` |
| 2 | Terminal/basic shell: `cd`, `ls`, `pwd`, `mkdir`, menjalankan `python3 file.py` | `setup/w0-2.py` |
| 3 | Editor & IDE (VS Code): ekstensi Python, menjalankan file, debug dasar | `setup/w0-3.py` |
| 4 | Git & GitHub dasar: `init`, `add`, `commit`, `push`, `.gitignore` | repo sendiri |
| 5 | REPL (`python3` di terminal), help, `dir()`, praktik mandiri | `setup/w0-5.py` |

✅ **Checklist Fase 0** (centang saat berhasil):
- [ ] `python3 --version` berjalan normal
- [ ] Bisa membuat file `.py` dan menjalankan lewat VS Code **dan** terminal
- [ ] Sudah melakukan commit + push pertama ke GitHub
- [ ] Paham perbedaan terminal, REPL, dan editor

---

<!-- ================= FAZA 1 ================= -->
## Fase 1 — Fundamental Python (Minggu 1–4)

**Tujuan:** Menguasai sintaks inti Python; bisa membuat program berlogika (input → proses → output).

### Minggu 1 — Dasar Sintaks
- `print()`, komentar, PEP 8 (aturan penulisan rapi)
- Variabel & aturan penamaan (`snake_case`)
- Tipe data dasar: `int`, `float`, `str`, `bool`
- Operasi aritmatika & operator pembanding
- Konversi tipe (`int()`, `str()`, `float()`)

**Latihan** (folder `python/src/fundamental/`):
- `w1-1.py` → cetak biodata singkat
- `w1-2.py` → kalkulator sederhana (+, -, *, /)
- `w1-3.py` → konversi suhu Celsius ↔ Fahrenheit

### Minggu 2 — String & Input
- String: indexing, slicing, metode (`upper`, `lower`, `split`, `join`, `replace`, `strip`, `format`, f-string)
- `input()` & validasi dasar
- Perulangan `for ... range`, `while`
- `break`, `continue`, `pass`

**Latihan:**
- `w2-1.py` → palindrome checker
- `w2-2.py` → hitung jumlah karakter & kata dalam kalimat
- `w2-3.py` → tabel perkalian 1–10
- `w2-4.py` → tebak angka acak (`random`)

### Minggu 3 — Percabangan (*Control Flow*)
- `if`, `elif`, `else` & operator logika (`and`, `or`, `not`)
- Comparison chaining & `in` operator
- Latihan problem solving & alur program (flowchart sederhana)

**Latihan:**
- `w3-1.py` → menentukan nilai huruf (A–E)
- `w3-2.py` → cek tahun kabisat
- `w3-3.py` → game tebak angka dengan batas kesempatan
- `w3-4.py` → program menu pilihan (kalkulator versi menu)

### Minggu 4 — Review + Proyek Mini #1
- Review semua materi minggu 1–3
- Latihan campuran & debugging (menemukan error umum)

**Proyek Mini #1:** `proyek/w4-average.py`
> Program "Pengelola Nilai Siswa" — input beberapa nilai, hitung rata-rata,
> tentukan nilai huruf, tampilkan pesan lulus/tidak.

**Checklist Fase 1:**
- [ ] Bisa menjelaskan perbedaan `int`, `float`, `str`, `bool`
- [ ] Lancar memakai f-string & slicing string
- [ ] Bisa menulis `for` & `while` tanpa mencontek
- [ ] Program bebas error untuk kasus sederhana

---
<!-- ================= FAZA 2 ================= -->
## Fase 2 — Struktur Data & Fungsi (Minggu 5–8)

**Tujuan:** Mengolah kumpulan data dengan struktur data bawaan Python dan menulis fungsi modular yang bisa dipakai ulang.

### Minggu 5 — List & Tuple
- `list`: membuat, index, slicing, menambah/menghapus (`append`, `insert`, `remove`, `pop`, `del`)
- Nested list / matriks
- `tuple` (immutable) & kapan menggunakannya
- Metode umum: `sort`, `reverse`, `count`, `index`

**Latihan** (folder `python/src/datastructure/`):
- `w5-1.py` → statistik daftar angka (min, max, sum, rata-rata)
- `w5-2.py` → hapus duplikat & urutkan
- `w5-3.py` → operasi matriks 3×3
- `w5-4.py` → tukar isi dua variabel ala Python (`a, b = b, a`)

### Minggu 6 — Dictionary & Set
- `dict`: key-value, menambah/mengubah/menghapus, iterasi (`items`, `keys`, `values`)
- `set`: operasi himpunan (union, intersection, difference) & menghapus duplikat
- Nested dict & menyimpan data terstruktur
- `defaultdict`, `Counter` dari modul `collections`

**Latihan:**
- `w6-1.py` → kamus kata slang/terjemahan
- `w6-2.py` → hitung frekuensi huruf/kata menggunakan `Counter`
- `w6-3.py` → data mahasiswa nested dict + pencarian
- `w6-4.py` → program manajemen kontak sederhana

### Minggu 7 — Fungsi Dasar (`python/src/function/`)
- `def`, parameter posisi & keyword, nilai default
- `return` vs `print`, multiple return values
- Scope: lokal vs global, `global`, `nonlocal`
- Fungsi memanggil fungsi lain; modularisasi kode

**Latihan:**
- `w7-1.py` → fungsi matematika (faktorial, fibonacci, luas bangun)
- `w7-2.py` → fungsi validasi input (angka vs teks)
- `w7-3.py` → fungsi pecah masalah besar menjadi fungsi-fungsi kecil
- `w7-4.py` → mini kalkulator berbasis menu yang dipisah tiap operasi jadikan fungsi

### Minggu 8 — Lambda, Comprehensions & Proyek Mini #2
- Lambda (`lambda x: x**2`)
- List/dict/set comprehension & conditional comprehension
- `map`, `filter`, `zip`, `enumerate`
- Pengenalan `*args` & `**kwargs` (pintasan ke Fase 4)

**Proyek Mini #2:** `proyek/w8-todolist.py`
> Program **To-Do List** di terminal: tambah, lihat, hapus, tandai selesai,
> disimpan dalam list of dict. (Tanpa file dulu — memori saja.)

**Checklist Fase 2:**
- [ ] Bisa memilih struktur data yang tepat: list vs tuple vs dict vs set
- [ ] Paham perbedaan fungsi dengan / tanpa `return`
- [ ] Bisa menulis list comprehension dengan kondisi
- [ ] Program to-do list berjalan tanpa bug

---

## Fase 3 — Pemrograman Berorientasi Objek (OOP) (Minggu 9–12)

**Tujuan:** Mendesain program dengan class; memahami 4 pilar OOP dan cara Python mengimplementasikannya.

### Minggu 9 — Class & Object Dasar (`python/src/oop/`)
- `class`, `__init__`, `self`, atribut & metode
- Object, instansiasi, dan cara menyimpan state
- `__str__` & `__repr__`

**Latihan:**
- `w9-1.py` → class `Mahasiswa` (nama, nim, nilai, metode `ipk()`)
- `w9-2.py` → class `RekeningBank` (setor, tarik, cek saldo)
- `w9-3.py` → class `Kalkulator` dengan state memory
- `w9-4.py` → proyek mini: sistem peminjaman buku perpustakaan

### Minggu 10 — Encapsulation & Properties
- Atribut publik, proteksi, privat (konvensi)
- `@property`, setter & getter
- `@classmethod` & `@staticmethod`
- Class variable vs instance variable

**Latihan:**
- `w10-1.py` → class `AkunBank` dengan validasi saldo melalui property
- `w10-2.py` → counter global pakai classmethod
- `w10-3.py` → class `Waktu` dengan property & validasi 0–23

### Minggu 11 — Inheritance & Polymorphism
- Pewarisan (`class B(A)`), `super()`, overriding
- Multiple inheritance & Method Resolution Order (MRO)
- Polymorphism: metode sama, perilaku berbeda
- Abstract Base Class (`abc`)

**Latihan:**
- `w11-1.py` → class `Hewan` → `Kucing`, `Anjing`, `Kambing` (suara berbeda / polymorphism)
- `w11-2.py` → class `Kendaraan` → `Mobil`, `Motor`, `Sepeda`
- `w11-3.py` → ABC `Shape` → `Lingkaran`, `Persegi` (hitung luas)

### Minggu 12 — Magic Methods, Dataclasses & Proyek Mini #3
- `__eq__`, `__lt__`, `__add__`, `__len__`, `__getitem__`, `__iter__`
- Operator overloading
- `@dataclass` untuk data sederhana

**Proyek Mini #3:** `proyek/w12-bank.py`
> **Sistem Bank Sederhana** dengan OOP penuh:
> class `Nasabah`, `Akun`, `Bank`; deposit, transfer antar akun,
> riwayat transaksi, dan overloading operator (e.g., menjumlahkan saldo 2 akun).

**Checklist Fase 3:**
- [ ] Bisa menjelaskan 4 pilar OOP dengan contoh sendiri
- [ ] Paham kapan pakai `@property`, `@classmethod`, `@staticmethod`
- [ ] Bisa merancang hierarki class yang benar (bukan asal pewarisan)
- [ ] Program bank berjalan lengkap

---
## Fase 4 — Intermediate (Minggu 13–16)

**Tujuan:** Menulis kode profesional: modular, tahan error, bisa baca/tulis data, dan teruji dengan test.

### Minggu 13 — Exception Handling
- `try`, `except`, `else`, `finally`
- Menangkap exception spesifik (`ValueError`, `TypeError`, `FileNotFoundError`, ...)
- Membuat custom exception (`class MyError(Exception)`)
- Raise exception; best practices

**Latihan** (folder `python/src/advanced/`):
- `w13-1.py` → kalkulator ramah error (tangkapi input salah)
- `w13-2.py` → validasi kode dengan custom exception
- `w13-3.py` → program menu yang tidak pernah crash

### Minggu 14 — File I/O & Formulir Data
- Membaca/menulis file teks: `open`, `with`, `read`, `write`, `seek`
- Mode `r/w/a/x`, encoding UTF-8
- Format data: CSV (`csv`), JSON (`json`), dan dasar XML
- Path & modul `os`, `pathlib`

**Latihan:**
- `w14-1.py` → simpan & baca catatan harian (file teks)
- `w14-2.py` → baca & olah data CSV (nilai siswa)
- `w14-3.py` → simpan/muat config JSON
- `w14-4.py` → perbaiki *Proyek Mini #2* menjadi to-do list **berbasis file**

### Minggu 15 — Decorators, Generators & Iterators
- `*args`, `**kwargs` dalam-dalam
- Generator: `yield`, `next`, `itertools`
- Decorator dasar: membungkus fungsi; `functools.wraps`
- Context manager: `with`, `contextlib`

**Latihan:**
- `w15-1.py` → generator deret fibonacci
- `w15-2.py` → decorator `@timer` dan `@debug`
- `w15-3.py` → chained generators (deret → filter → map)
- `w15-4.py` → context manager timer sendiri

### Minggu 16 — Modules, Packages & Testing
- Struktur package (`__init__.py`, relative import)
- `if __name__ == "__main__":`
- Virtual environment (`venv`) & `pip`, `requirements.txt`
- Unit test dengan `unittest` & `pytest`
- `assert`

**Latihan:**
- `w16-1.py` → pecah *Proyek Mini #3* menjadi package (`bank/`) dengan modul terpisah
- `w16-test.py` → pytest untuk fungsi bank (deposit, transfer, saldo negatif)

**Proyek Mini #4:** `proyek/w16-champion.py`
> **Aplikasi Data Pemenang**: baca data CSV, hitung statistik, simpan hasil ke JSON,
> semua fungsi dipecah dalam modul, dan punya unit test minimal 5 kasus.

**Checklist Fase 4:**
- [ ] Program tidak pernah crash tanpa pesan yang jelas
- [ ] Bisa baca/tulis file CSV & JSON
- [ ] Paham apa itu decorator & generator (bisa menjelaskan dengan contoh sendiri)
- [ ] Punya minimal 1 package + test yang lewat

---

## ⚡ Fase 5 — Advanced (Minggu 17–20)

**Tujuan:** Memanfaatkan fitur lanjutan bahasa (concurrency, asyncio, typing, functional programming) dan library populer.

### Minggu 17 — Functional Programming & Refactoring
- `map`, `filter`, `reduce` dalam-dalam
- First-class function, closure, & partial (`functools.partial`)
- Pipeline/function composition
- Refactoring: fungsi kecil, DRY, docstring & type hints dasar

**Latihan:**
- `w17-1.py` → data pipeline: baca list → filter → transform → urut → agregasi (tanpa loop)
- `w17-2.py` → refactor kode lama yang masih `spaghetti`
- `w17-3.py` → latihan partial & curried functions

### Minggu 18 — Concurrency: Threading & Multiprocessing
- Threading vs multiprocessing vs asyncio (kapan pakai yang mana)
- `threading.Thread`, `global` lock, race condition
- `multiprocessing.Pool` & `Process`
- GIL: apa itu dan dampaknya

**Latihan:**
- `w18-1.py` → download file paralel dengan thread pool
- `w18-2.py` → hitung CPU-bound parallel dengan multiprocessing
- `w18-3.py` → race condition & solusi `Lock`

### Minggu 19 — Asyncio & I/O Async
- `asyncio`: `async def`, `await`, `asyncio.run`
- `create_task`, `gather`, timeout
- `aiohttp` / `httpx` untuk request HTTP asynchronous

**Latihan:**
- `w19-1.py` → fetch beberapa API sekaligus dengan `gather`
- `w19-2.py` → mini web scraper async
- `w19-3.py` → producer-consumer dengan asyncio queue

### Minggu 20 — Type Hints, Git Lanjutan & Proyek Mini #5
- Deep type hints: `Optional`, `Union`, `List`, `Dict`, `Callable`, `TypeVar`, `Protocol`
- Pengecekan statis dengan `mypy`
- Git lanjutan: branch, merge, rebase, pull request, resolve conflict

**Proyek Mini #5:** `proyek/w20-async-fetch.py`
> **Tool Fetch Data**: mengambil data dari beberapa endpoint publik secara
> async, menyimpan ke JSON, deduplikasi, dengan type hints penuh,
> dijalankan di branch fitur via git lalu PR.

**Checklist Fase 5:**
- [ ] Bisa menjelaskan kapan pakai threading vs multiprocessing vs asyncio
- [ ] Bisa menulis async code yang benar (tanpa blocking)
- [ ] Punya kode dengan type hints & lolos `mypy`
- [ ] Sudah pernah merge branch & mengatasi conflict

---
## Fase 6 — Expert (Minggu 21–24+)

**Tujuan:** Memahami cara kerja internal Python, menulis kode cepat, membuat library sendiri, hingga berkontribusi ke open-source.

### Minggu 21 — Metaclasses, Descriptors & *Dunder*
- Kimia class: metaclass (`type`), `__new__` vs `__init__`
- Descriptor protocol (`__get__`, `__set__`, `__delete__`)
- Magic methods lanjutan: `__enter__`, `__exit__`, `__call__`, `__slots__`
- Kapan (dan kapan **jangan**) memakai metaclass — "pythonic way"

**Latihan** (folder `python/src/expert/`):
- `w21-1.py` → custom metaclass yang mensyuting class baru (auto-register class)
- `w21-2.py` → descriptor untuk atribut validasi reusable
- `w21-3.py` → class `Callable` (`__call__`) untuk pattern strategy sederhana

### Minggu 22 — Optimasi, Profiling & Memori
- Profiling: `cProfile`, `timeit`, `memory_profiler`
- Optimasi: pilih struktur data, caching (`functools.lru_cache`), menghindari anti-pattern
- Memahami `id()`, mutable vs immutable, references, garbage collection, `gc`
- `__slots__` untuk hemat memori; big-O untuk memilih algoritma

**Latihan:**
- `w22-1.py` → benchmark 3 solusi problem yang sama (naive vs optimized)
- `w22-2.py` → cache memoization fibonacci (recursive vs recursive + lru_cache)
- `w22-3.py` → profil program yang lambat, temukan bottleneck, tulis sebelum/sesudah

### Minggu 23 — Packaging, C-Extensions & Library Populer
- Membuat package distribusi: `pyproject.toml`, `setuptools`/`hatchling`
- Publish ke PyPI (TestPyPI dulu); versioning semver
- `*.so`/Cython/Numba untuk bagian yang butuh kecepatan
- Library profesional: `requests`/`httpx`, `pandas`, `numpy`, `fastapi`, `pydantic`, `click`/`typer`, `rich`

**Latihan:**
- `w23-1.py` → buat package kecil (mis. `mytextutils`) + upload ke TestPyPI
- `w23-2.py` → pipeline analisis data dengan pandas (10 baris vs 50 baris manual)
- `w23-3.py` → mini REST API dengan FastAPI + validasi pydantic

### Minggu 24 — Design Patterns, Zaman & Proyek Akhir
- Design patterns Pythonic: Singleton (dengan cara Python), Factory, Repository, Observer, Template Method
- Clean code: SOLID, separation of concerns, dokumentasi (`docstring`, `pydoc`)
- Kontribusi open-source: baca kode proyek besar (mis. FastAPI, Django), cari issue `good-first-issue`, buat PR

**Proyek Akhir (Capstone):** `proyek/capstone.py`
> **Aplikasi Library Management** lengkap:
> - CRUD buku/anggota dengan **persistensi file/DB**
> - CLI yang rapi (`typer`/`click`) + **logging**
> - **Unit test** > 15 kasus, type hints, docstring
> - Concurrency/async untuk fitur export laporan
> - Dikemas sebagai package → installable
> - Semua via git + GitHub (branch, PR, README, CI opsional)

**Checklist Fase 6:**
- [ ] Bisa menjelaskan perbedaan `__new__` dan `__init__`
- [ ] Bisa memprofil dan mengoptimalkan program dengan dasar keputusan
- [ ] Punya package sendiri di PyPI (nama apa pun, sekalipun kecil)
- [ ] Pernah membuka PR yang di-*review* orang lain di open-source

---

## Rangkuman Timeline

| Minggu | Fokus | File Utama |
|--------|-------|------------|
| 0 | Setup | `setup/` |
| 1–4 | Fundamental | `fundamental/` |
| 5–8 | Struktur Data & Fungsi | `datastructure/`, `function/` |
| 9–12 | OOP | `oop/` |
| 13–16 | Intermediate | `advanced/` |
| 17–20 | Advanced | `advanced/` |
| 21–24+ | Expert | `expert/` |
| Akhir | Capstone | `proyek/` |

---

##  Sumber Belajar Rekomendasi

### Dokumentasi Resmi
- [Python.org Tutorial](https://docs.python.org/3/tutorial/) — wajib
- [Python Docs — The Python Standard Library](https://docs.python.org/3/library/)
- [PEP 8 — Panduan Gaya](https://peps.python.org/pep-0008/)
- [Python Data Model](https://docs.python.org/3/reference/datamodel.html) — untuk Fase 6

### Buku
- *Automate the Boring Stuff with Python* — Al Sweigart (gratis online)
- *Python Crash Course* — Eric Matthes
- *Fluent Python* — Luciano Ramalho (intermediate–expert, wajib)
- *Effective Python* — Brett Slatkin

### Kursus/Video (Bahasa Indonesia)
- Kelas terbuka / playlist YouTube Python dasar
- Dicoding, Udemy, FreeCodeCamp Python curriculum

### Latihan Soal (habituasi problem solving)
- [Exercism Python Track](https://exercism.org/tracks/python)
- [codewars](https://www.codewars.com), [HackerRank](https://www.hackerrank.com), [LeetCode](https://leetcode.com)

---

## Tips & Habit Belajar

1. **Konsisten > intensitas** — 25 menit/hari lebih baik dari 5 jam sekaligus.
2. **Ketik, jangan copy-paste** — mengetik sendiri membentuk memori otot sintaks.
3. **Jelaskan ulang** — ajarkan ke teman, atau *rubber duck debugging*: jelaskan kode pada bebek karet.
4. **Baca error sampai habis** — traceback adalah petunjuk terbaik.
5. **Fokus satu topik** — jangan loncat ke framework sebelum Fase 4 selesai.
6. **Tulis catatan** — buat `catatan.md` per fase berisi *gotcha* yang kamu temui.
7. **Github setiap sesi** — commit kecil-kecil, bangun kebiasaan.
8. **Debug mandiri** — coba 15–30 menit sendiri sebelum bertanya.

---

##  Catatan Akhir

Apabila menemui konflik ide antara "jalan cepat" (copy-paste dari AI) dan
"memahami" — pilih memahami. Roadmap ini berjalan berdampingan dengan praktik
menulis kode dari nol. Setiap fase memiliki **checklist** di atas;
jangan lanjut fase berikutnya sebelum checklist fase sebelumnya **dicentang semua**.

>  **Selamat belajar, selamat berkode. Setiap kode yang kamu tulis hari ini
> adalah fondasi untuk proyek besar yang kamu impikan besok.**
