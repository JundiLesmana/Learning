#  Jalankan file Python dari folder src/ lewat perintah make.
#
#  make                                  -> bantuan (daftar perintah)
#  make list                             -> daftar semua file .py di src/
#  make run FILE=linear-regression/sp-w8 -> jalankan src/linear-regression/sp-w8.py
#  make run FILE=src/linear-regression/sp-w8.py  -> boleh pakai path lengkap
#  make setup                            -> buat venv + install dependencies (requirements.txt)
#  make clean                            -> hapus __pycache__ & hasil plot (output/)

VENV    := .venv
PY      := $(if $(wildcard $(VENV)/bin/python3),$(VENV)/bin/python3,python3)

all:
	@echo "Perintah yang tersedia:"
	@echo "  make list                          -> daftar semua file .py"
	@echo "  make run FILE=linear-regression/sp-w8"
	@echo "  make setup                         -> install dependencies (wajib sekali)"
	@echo "  make clean                         -> hapus __pycache__ & output/"

run:
	@norm=$$(echo "$(FILE)" | sed -E 's@^src/@@; s@.*/src/@@; s@\.py$$@@'); \
	if [ -z "$$norm" ]; then \
		echo "Contoh: make run FILE=linear-regression/sp-w8"; \
		exit 1; \
	fi; \
	src="src/$$norm.py"; \
	if [ ! -f "$$src" ]; then \
		echo "File tidak ditemukan: $$src"; \
		echo "Cek daftar file: make list"; \
		exit 1; \
	fi; \
	echo "▶ Menjalankan $$src ..."; \
	$(PY) "$$src"

list:
	@files=$$(find src -name '*.py' 2>/dev/null | sed 's|^src/||; s|\.py$$||' | sort); \
	if [ -z "$$files" ]; then echo "Belum ada file .py di src/"; else echo "$$files"; fi

setup:
	@command -v python3 >/dev/null 2>&1 || { echo "python3 tidak ditemukan"; exit 1; }
	python3 -m venv $(VENV)
	$(VENV)/bin/python3 -m pip install --upgrade pip
	$(VENV)/bin/python3 -m pip install -r requirements.txt
	@echo ""
	@echo "✔ Dependencies terinstall. Jalankan: make run FILE=linear-regression/sp-w8"

clean:
	@find src -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	@rm -rf output
	@echo "✔ Bersih"

.PHONY: all run list setup clean