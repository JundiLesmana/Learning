#include <iostream>

using namespace std;

const int MAX = 5;

class Deque {
    int data[MAX];
    int front, rear, count; // count digunakan untuk mempermudah cek penuh/kosong

public:
    Deque() : front(0), rear(0), count(0) {}

    bool isEmpty() const { return count == 0; }
    bool isFull()  const { return count == MAX; }

    // 1. INSERT DEPAN
    void insertFront(int val) {
        if (isFull()) { cout << "[FULL] Deque penuh!\n"; return; }
        if (!isEmpty()) front = (front - 1 + MAX) % MAX; // Mundur 1 langkah (melingkar)
        data[front] = val;
        count++;
    }

    // 2. INSERT BELAKANG
    void insertRear(int val) {
        if (isFull()) { cout << "[FULL] Deque penuh!\n"; return; }
        data[rear] = val;
        rear = (rear + 1) % MAX; // Maju 1 langkah (melingkar)
        count++;
    }

    // 3. DELETE DEPAN
    void deleteFront() {
        if (isEmpty()) { cout << "[EMPTY] Deque kosong!\n"; return; }
        cout << "Del Front: " << data[front] << "\n";
        front = (front + 1) % MAX;
        count--;
    }

    // 4. DELETE BELAKANG
    void deleteRear() {
        if (isEmpty()) { cout << "[EMPTY] Deque kosong!\n"; return; }
        rear = (rear - 1 + MAX) % MAX; // Mundur 1 langkah (melingkar)
        cout << "Del Rear: " << data[rear] << "\n";
        count--;
    }

    // TAMPILKAN DATA
    void display() const {
        if (isEmpty()) { cout << "[EMPTY] Deque kosong.\n"; return; }
        cout << "Deque: ";
        for (int i = 0; i < count; i++) {
            cout << data[(front + i) % MAX] << " ";
        }
        cout << "\n";
    }
};

int main() {
    Deque d;

    d.insertRear(10);
    d.insertRear(20);
    d.insertFront(5);
    d.display();        // Deque: 5 10 20

    d.deleteRear();     // Del Rear: 20
    d.deleteFront();    // Del Front: 5
    d.display();        // Deque: 10

    d.insertFront(99);
    d.insertFront(88);
    d.display();        // Deque: 88 99 10

    return 0;
}