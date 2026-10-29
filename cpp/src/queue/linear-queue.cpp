#include <iostream>

using namespace std;

const int MAX = 5; // Kapasitas maksimal antrian

class LinearQueue {
    int data[MAX];
    int front, rear;

public:
    LinearQueue() : front(-1), rear(-1) {}

    bool isEmpty() const { return front == -1 || front > rear; }
    bool isFull()  const { return rear == MAX - 1; }

    // Tambah data di belakang (rear)
    void enqueue(int val) {
        if (isFull()) {
            cout << "[FULL] Antrian penuh!\n";
            return;
        }
        if (isEmpty()) front = 0; // Inisialisasi front saat antrian kosong
        data[++rear] = val;
    }

    // Hapus data di depan (front)
    void dequeue() {
        if (isEmpty()) {
            cout << "[EMPTY] Antrian kosong!\n";
            return;
        }
        cout << "Dequeue: " << data[front++] << "\n";
        
        // Reset pointer jika antrian menjadi kosong setelah dequeue
        if (front > rear) {
            front = -1;
            rear = -1;
        }
    }

    // Tampilkan isi antrian
    void display() const {
        if (isEmpty()) {
            cout << "[EMPTY] Antrian kosong.\n";
            return;
        }
        cout << "Queue (Front -> Rear): ";
        for (int i = front; i <= rear; i++) {
            cout << data[i] << " ";
        }
        cout << "\n";
    }
};

int main() {
    LinearQueue q;

    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);
    q.display();        // Queue: 10 20 30

    q.dequeue();        // Dequeue: 10
    q.dequeue();        // Dequeue: 20
    q.display();        // Queue: 30

    q.enqueue(40);
    q.enqueue(50);
    q.enqueue(60);      // [FULL] Antrian penuh! (karena rear sudah di index 4)
    
    q.display();        // Queue: 30 40 50

    return 0;
}