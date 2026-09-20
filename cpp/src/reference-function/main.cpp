#include <iostream>
using namespace std;

void fungsi (int &b){
    cout << "alamat b: " << &b << endl;
    cout << "nilai b: " << b << endl;
}

void kuadrat (int &nilaiRef){
    nilaiRef = nilaiRef * nilaiRef;
}

int main(){

    int a = 10;
    cout << endl;
    cout << "alamat a:" << &a << endl;
    cout << "nilai a: " << a << endl;

    fungsi(a);
    kuadrat(a);
    cout << "nilai a: " << a << endl;

    cin.get();
    return 0;
}