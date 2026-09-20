#include <iostream>
using namespace std;

int main() {
    int a = 10;
    int *Ptr = &a; // Pointer to integer, storing the address of a

    cout << "Value of a: " << a << endl;
    cout << "Address of a: " << &a << endl;
    cout << "Value of Ptr: " << Ptr << endl;
    cout << "Value pointed to by Ptr: " << *Ptr << endl;
    return 0;
}