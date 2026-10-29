#include <iostream>

using namespace std;

const int N = 10;

class DoubleStack {
    int data[N];
    int top1, top2;

public:
    DoubleStack() : top1(-1), top2(N) {}

    bool isFull()   const { return top1 + 1 == top2; }
    bool empty1()   const { return top1 == -1; }
    bool empty2()   const { return top2 == N;  }

    void push(int s, int val) {
        if (isFull()) { cout << "[FULL] Array penuh!\n"; return; }
        if (s == 1) data[++top1] = val;   // tumbuh ke kanan
        else        data[--top2] = val;    // tumbuh ke kiri
    }

    void pop(int s) {
        if (s == 1) {
            if (empty1()) { cout << "[Stack 1 kosong]\n"; return; }
            cout << "Pop Stack1: " << data[top1--] << "\n";
        } else {
            if (empty2()) { cout << "[Stack 2 kosong]\n"; return; }
            cout << "Pop Stack2: " << data[top2++] << "\n";
        }
    }

    void display() const {
        cout << "S1: ";
        for (int i = 0; i <= top1; i++) cout << data[i] << " ";
        cout << " | S2: ";
        for (int i = N - 1; i >= top2; i--) cout << data[i] << " ";
        cout << "\n";
    }
};

int main() {
    DoubleStack ds;

    ds.push(1, 10);
    ds.push(1, 20);
    ds.push(2, 30);
    ds.push(2, 40);
    ds.display();        

    ds.pop(1);          
    ds.pop(2);          
    ds.display();       
    return 0;
}