"""
sp-w8.py — Visualisasi Regresi Linier Berganda: Prediksi Harga Rumah

Program ini terhubung dengan Makefile dan menerima input interaktif dari terminal:

  1. Jumlah data (n) — tekan Enter untuk memakai data contoh
  2. Untuk setiap data:  Luas (m2), Usia (tahun), Harga (juta)  → dipisah koma
  3. Opsional: prediksi harga untuk rumah baru (masukkan Luas, Usia)

Cara menjalankan:
    make run FILE=linear-regression/sp-w8

Contoh input satu baris data:
    Data  1 (Luas, Usia, Harga): 100, 5, 800
"""

import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 — wajib untuk projection='3d'
import statsmodels.api as sm

# Data contoh (dipakai bila user tidak mengisi jumlah data)
CONTOH = {
    "X1": [100, 120, 150, 130, 160],    # Luas rumah (m2)
    "X2": [5, 10, 5, 15, 10],           # Usia rumah (tahun)
    "Y":  [800, 900, 1100, 950, 1200],  # Harga rumah (juta rupiah)
}


def baca_titik(prompt):
    """Baca satu baris 'Luas, Usia, Harga'. Kembalikan [luas, usia, harga]."""
    while True:
        teks = input(prompt).strip()
        bagian = [b.strip() for b in teks.split(",")]
        if len(bagian) != 3:
            print("  ⚠ Format harus: Luas, Usia, Harga  (contoh: 120, 5, 900)")
            continue
        try:
            return [float(b) for b in bagian]
        except ValueError:
            print("  ⚠ Semua nilai harus angka. Contoh: 120, 5, 900")


def input_data():
    """Minta data dari terminal. Kembalikan tuple (X1, X2, Y) berupa np.array."""
    n_teks = input("\nJumlah data (Enter = pakai data contoh): ").strip()

    if n_teks == "":
        print("  → Memakai data contoh (5 data).")
        return (np.array(CONTOH["X1"], dtype=float),
                np.array(CONTOH["X2"], dtype=float),
                np.array(CONTOH["Y"], dtype=float))

    try:
        n = int(n_teks)
    except ValueError:
        print("  ⚠ Jumlah data harus bilangan bulat. Memakai data contoh.")
        return (np.array(CONTOH["X1"], dtype=float),
                np.array(CONTOH["X2"], dtype=float),
                np.array(CONTOH["Y"], dtype=float))

    if n < 3:
        print("  ⚠ Minimal 3 data untuk regresi. Memakai data contoh.")
        return (np.array(CONTOH["X1"], dtype=float),
                np.array(CONTOH["X2"], dtype=float),
                np.array(CONTOH["Y"], dtype=float))

    x1, x2, y = [], [], []
    for i in range(1, n + 1):
        luas, usia, harga = baca_titik(f"  Data {i:>2d} (Luas, Usia, Harga): ")
        x1.append(luas)
        x2.append(usia)
        y.append(harga)

    return np.array(x1), np.array(x2), np.array(y)


def prediksi(model):
    """Loop prediksi harga rumah baru sampai user mengakhiri (tekan Enter kosong)."""
    print("\n--- Prediksi Harga Rumah Baru (tekan Enter tanpa isi untuk selesai) ---")
    while True:
        luas_teks = input("  Luas rumah (m2)   [Enter = selesai]: ").strip()
        if luas_teks == "":
            print("  Selesai.")
            break
        usia_teks = input("  Usia rumah (tahun): ").strip()
        if usia_teks == "":
            print("  Selesai.")
            break
        try:
            luas, usia = float(luas_teks), float(usia_teks)
        except ValueError:
            print("  ⚠ Masukkan angka yang valid.")
            continue

        b0, b1, b2 = model.params
        harga = b0 + b1 * luas + b2 * usia
        print(f"  → Prediksi Harga: Rp {harga:,.2f} juta")


def tampilkan_plot(X1, X2, Y, model):
    """Gambar bidang regresi 3D beserta titik data asli."""
    pad_x = max(5.0, (X1.max() - X1.min()) * 0.15)
    pad_y = max(0.5, (X2.max() - X2.min()) * 0.25)

    # Membuat meshgrid untuk bidang regresi (rentang otomatis dari data)
    x1_surf = np.linspace(X1.min() - pad_x, X1.max() + pad_x, 10)
    x2_surf = np.linspace(X2.min() - pad_y, X2.max() + pad_y, 10)
    X1_surf, X2_surf = np.meshgrid(x1_surf, x2_surf)

    # Hitung nilai Y prediksi untuk permukaan: Y = b0 + b1*X1 + b2*X2
    b0, b1, b2 = model.params
    Y_surf = b0 + b1 * X1_surf + b2 * X2_surf

    fig = plt.figure(figsize=(10, 7))
    ax = fig.add_subplot(111, projection='3d')

    # Plot titik data asli
    ax.scatter(X1, X2, Y, color='red', s=50, label='Data Asli')

    # Plot bidang regresi
    ax.plot_surface(X1_surf, X2_surf, Y_surf, alpha=0.5, color='blue', edgecolor='none')

    ax.set_xlabel('Luas (X1)')
    ax.set_ylabel('Usia (X2)')
    ax.set_zlabel('Harga (Y)')
    ax.set_title('Visualisasi Regresi Linier Berganda: Prediksi Harga Rumah')
    ax.legend()

    try:
        plt.show()  # menunggu jendela plot ditutup
    except Exception:
        # Tidak ada tampilan grafis (mis. lewat SSH) → simpan sebagai gambar.
        import os
        os.makedirs("output", exist_ok=True)
        fig.savefig("output/hasil-regresi.png", dpi=120)
        print("  [Jendela plot tidak tersedia → gambar disimpan ke output/hasil-regresi.png]")


def main():
    print("=" * 60)
    print("REGRESI LINIER BERGANDA — PREDIKSI HARGA RUMAH")
    print("Variabel: X1 = Luas (m2), X2 = Usia (tahun), Y = Harga (juta)")
    print("=" * 60)

    X1, X2, Y = input_data()

    # Model regresi linier berganda: Y = b0 + b1*X1 + b2*X2
    X = sm.add_constant(np.column_stack((X1, X2)))
    model = sm.OLS(Y, X).fit()

    b0, b1, b2 = model.params
    print("\nPersamaan Regresi:")
    print(f"  Y = {b0:.2f} + {b1:.2f}*X1 + ({b2:.2f})*X2")
    print(f"  R² = {model.rsquared:.4f}")

    prediksi(model)
    tampilkan_plot(X1, X2, Y, model)


if __name__ == "__main__":
    main()