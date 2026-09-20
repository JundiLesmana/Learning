#include <iostream>
using namespace std;

int main(){
    //variabel
    int a = 10;

    cout << "Nilai a: " << a << endl;
    cout << "Alamat a: " << &a << endl;
    
    //referece
    int &b = a;
    b = 20;
    cout << "Nilai b: " << b << endl;
    cout << "Alamat b: " << &b << endl;
    
    int &c = a;
    c = 30;
    cout <<"Nilai c: " << c << endl;
    cout <<"Alamat c: " << &c << endl;
        

    cin.get();
    return 0;
}