#include <iostream>
#include <string>

using namespace std;

#define MAX 10

class Queue {
private:
    int front, rear;
    string ele[MAX];

public:
    Queue() {
        front = 0;
        rear = -1;
    }

    int isFull() {
        return (rear == MAX - 1) ? 1 : 0;
    }

    int isEmpty() {
        return (front == rear + 1) ? 1 : 0;
    }

    void insertQueue(string item) {
        if (isFull()) {
            cout << "\n[Queue OverFlow] Antrian sudah penuh!" << endl;
            return;
        }
        ele[++rear] = item;
        cout << "  -> Sukses: " << item << " masuk ke dalam antrian." << endl;
    }

    int deleteQueue(string *item) {
        if (isEmpty()) {
            cout << "\n[Queue Underflow] Antrian sudah kosong!" << endl;
            return -1;
        }
        *item = ele[front++];
        return 0;
    }

    void displayQueue() {
        if (isEmpty()) {
            cout << "  [Sisa Antrian: KOSONG - Tidak ada pelanggan lagi]" << endl;
        } else {
            cout << "  [Sisa Antrian:";
            int nomor = 1;
            for (int i = front; i <= rear; i++) {
                cout << " (" << nomor << ") " << ele[i];
                nomor++;
            }
            cout << " ]" << endl;
        }
    }
};

int main() {
    Queue q;
    string namaPelanggan;

    cout << "========================================" << endl;
    cout << "       SISTEM ANTRIAN KASIR (FIFO)      " << endl;
    cout << "========================================" << endl;
    cout << "Kapasitas Antrian: " << MAX << " pelanggan." << endl;
    cout << "Maksimal input: 10 record." << endl;
    cout << "----------------------------------------" << endl;

    for (int i = 1; i <= 10; i++) {
        if (q.isFull()) {
            cout << "\n>> [PERINGATAN] Antrian PENUH ."
                 << " Proses input dihentikan pada record ke-" << (i - 1) << "." << endl;
            break;
        }

        cout << "\n[Pelanggan ke-" << i << "] Masukkan Nama: ";
        getline(cin >> ws, namaPelanggan);

        if (!namaPelanggan.empty()) {
            q.insertQueue(namaPelanggan);
        } else {
            cout << "  -> Nama tidak boleh kosong. Silakan ulangi." << endl;
            i--;
        }
    }

    cout << "\n========================================" << endl;
    cout << "         MULAI PROSES PELAYANAN         " << endl;
    cout << "========================================" << endl;
    cout << "Tekan [Enter] setiap kali kasir siap melayani pelanggan." << endl;

    for (int i = 1; i <= 10; i++) {
        if (q.isEmpty()) {
            cout << "\n>> [INFORMASI] Antrian KOSONG."
                 << " Tidak ada pelanggan lagi. Proses dihentikan." << endl;
            break;
        }

        cout << "\n[Pelayanan ke-" << i << "] Siap melayani pelanggan berikutnya...";
        cin.get();

        if (q.deleteQueue(&namaPelanggan) == 0) {
            cout << "  -> SUKSES: " << namaPelanggan << " telah selesai dilayani." << endl;
            q.displayQueue();
        }
    }

    cout << "\n========================================" << endl;
    cout << "         SISTEM KASIR DITUTUP           " << endl;
    cout << "========================================" << endl;

    cout << "Tekan [Enter] untuk keluar...";
    cin.get();

    return 0;
}