
SRC_DIR="src"
BIN_DIR="bin"

mapfile -t files < <(find "$SRC_DIR" -name '*.cpp' | sort)

find_exact() {
  local user="$1" cand
  for cand in "$user" "$user.cpp" "$SRC_DIR/$user" "$SRC_DIR/$user.cpp"; do
    if [ -f "$cand" ]; then
      echo "$cand"
      return 0
    fi
  done
  return 1
}

#Tentukan file yang mau dijalankan
if [ $# -eq 0 ]; then
  if [ ${#files[@]} -eq 0 ]; then
    echo "❌ Tidak ada file .cpp di folder $SRC_DIR/"
    exit 1
  fi

  echo "Pilih file yang mau di-compile & dijalankan:"
  i=1
  for f in "${files[@]}"; do
    printf "  %2d) %s\n" "$i" "${f#src/}"
    i=$((i + 1))
  done
  printf "Masukkan nomor (1-%d): " $((i - 1))
  read -r choice

  if ! [[ "$choice" =~ ^[0-9]+$ ]] || [ "$choice" -lt 1 ] || [ "$choice" -gt $((i - 1)) ]; then
    echo "❌ Nomor tidak valid."
    exit 1
  fi
  FILE="${files[$((choice - 1))]}"
else
  FILE=$(find_exact "$1") || {
    echo "❌ File '$1' tidak ditemukan. Coba ./run.sh untuk melihat daftar."
    exit 1
  }
fi

RELPATH=${FILE#src/}        
NAME=${RELPATH%.cpp}          
OUT="$BIN_DIR/$NAME"
mkdir -p "$(dirname "$OUT")"

echo "▶ Compile: $FILE"
g++ -std=c++17 -Wall -Wextra -g "$FILE" -o "$OUT" || { echo "❌ Compile gagal."; exit 1; }
echo "✔ Output : $OUT"
echo "✔ Cek    : menemukan file yang sama seperti 'make list'"

echo "▶ Jalankan..."
"$OUT"