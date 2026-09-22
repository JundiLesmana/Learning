#include <iostream>
#include <limits> 
using namespace std;

#define MAX 10

class Queue {
private:
    int front, rear;
    int ele[MAX];

public:
    // Inisialisasi queue
    Queue() {
        front = 0;
        rear  = -1;
    }

    int  isFull();
    int  isEmpty();
    void insertQueue(int item);
    int  deleteQueue(int *item);
};

// Cek penuh: R = n - 1 
int Queue::isFull() {
    int full = 0;
    if (rear == MAX - 1)
        full = 1;
    return full;
}

// Cek kosong: F = R + 1 
int Queue::isEmpty() {
    int empty = 0;
    if (front == rear + 1)
        empty = 1;
    return empty;
}

// Insert: Q[++R] = x 
void Queue::insertQueue(int item) {
    if (isFull()) {
        cout << "\nQueue OverFlow" << endl;
        return;
    }
    ele[++rear] = item;
    cout << "  -> Pelanggan ID " << item << " masuk antrian." << endl;
}

// Delete: x = Q[F++] 
int Queue::deleteQueue(int *item) {
    if (isEmpty()) {
        cout << "\nQueue Underflow" << endl;
        return -1;
    }
    *item = ele[front++];
    return 0;
}

int main() {
    Queue q; 
    int item = 0;

    cout << "\n--- Persiapan: Membuka Antrian ---" << endl;
    cout << "Sistem mencatat 3 pelanggan awal masuk: 100, 200, 300" << endl;
    q.insertQueue(100);
    q.insertQueue(200);
    q.insertQueue(300);

    cout << "MULAI PROSES PELAYANAN (DELETE)" << endl;
    cout << "Maksimal 10 pelanggan akan diproses." << endl;
    cout << "Tekan [Enter] setiap kali kasir siap melayani pelanggan berikutnya." << endl;
    cout << endl;

    cin.ignore(numeric_limits<streamsize>::max(), '\n');

    for (int i = 1; i <= 10; i++) {
        if (q.isEmpty()) {
            cout << "\n>> [INFORMASI] Antrian sudah KOSONG (F = R+1)."
                 << " Tidak ada pelanggan lagi untuk dilayani."
                 << " Proses dihentikan pada percobaan ke-" << i << " dari 10." << endl;
            break; 
        }
        
        cout << "\n[Pelanggan ke-" << i << "] Antrian berisi data. Siap untuk dilayani." << endl;
        cout << "Tekan [Enter] untuk memanggil dan melayani pelanggan ini...";
        cin.get();

        if (q.deleteQueue(&item) == 0) {
            cout << "  -> SUKSES: Pelanggan dengan ID " << item 
                 << " telah selesai dilayani dan keluar dari antrian." << endl;
        }
    }

    cout << "\n========================================" << endl;
    cout << "Proses Pelayanan Selesai" << endl;
    
    cout << "Tekan [Enter] untuk menutup aplikasi...";
    cin.get(); 
    
    return 0;
}