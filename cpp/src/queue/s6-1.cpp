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
    cout << "  -> Sukses: Pelanggan dengan ID " << item << " masuk antrian." << endl;
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
    int item;

    cout << "    SIMULASI ANTRIAN KASIR (FIFO)       " << endl;
    cout << endl;
    cout << "Kapasitas Antrian (MAX) = " << MAX << " pelanggan." << endl;
    cout << "Silakan masukkan data pelanggan (Maksimal 10 record)." << endl;
    cout << "----------------------------------------" << endl;

    for (int i = 1; i <= 10; i++) {
        if (q.isFull()) {
            cout << "\n>> [PERINGATAN] Antrian sudah PENUH (R = n-1)."
                 << " Proses pengisian dihentikan pada record ke-" << (i - 1)
                 << " dari 10." << endl;
            break; 
        }
        
        cout << "\n[Pelanggan ke-" << i << "] Masukkan ID/Nomor Transaksi: ";
        cin >> item;
        
        q.insertQueue(item);
    }

    cout << "\n========================================" << endl;
    cout << "Proses Input Antrian Selesai" << endl;
    
    cin.ignore(numeric_limits<streamsize>::max(), '\n');
    
    cout << "Tekan Enter untuk keluar...";
    cin.get(); 
    
    return 0;
}