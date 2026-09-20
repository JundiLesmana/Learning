#  Compile & run C++17 source code from src/ folder.
#  make list                            -> lihat semua file yang tersedia
#  make run FILE=pointer/main           -> compile & jalankan src/pointer/main.cpp
#  make run FILE=reference/main         -> compile & jalankan src/reference/main.cpp
#  make run FILE=src/week-1/main.cpp    -> boleh pakai path lengkap juga
#  make build FILE=week-1/main          -> compile saja (tanpa run)
#  make clean                           -> hapus hasil build

CXX      := g++
CXXFLAGS := -std=c++17 -Wall -Wextra -g
FILE     ?= main

SRC_NORM := $(basename $(patsubst src/%,%,$(FILE)))
SRC      := src/$(SRC_NORM).cpp
OUT      := bin/$(SRC_NORM)

# make prompt
all:
	@echo "Gunakan salah satu perintah:"
	@echo "  make list                       -> lihat daftar file yang tersedia"
	@echo "  make run FILE=pointer/main      -> compile & jalankan src/pointer/main.cpp"
	@echo "  make run FILE=reference/main    -> compile & jalankan src/reference/main.cpp"
	@echo "  make build FILE=week-1/main     -> compile saja (tanpa run)"
	@echo "  make clean                      -> hapus hasil build"

build: $(OUT)

$(OUT): $(SRC)
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -o $@ $(SRC)

run: $(OUT)
	@echo "▶ Menjalankan $(OUT) ..."
	./$(OUT)

list:
	@find src -name '*.cpp' 2>/dev/null | sed 's|^src/||; s|\.cpp$$||' | sort || echo "Belum ada file .cpp di src/"

clean:
	rm -rf bin
	@echo "✔ Bersih"

.PHONY: all build run list clean