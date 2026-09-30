#include <iostream>
#include <array>
using namespace std;
int main() {
 array<int,10> arreglo={1,2,3,4,5,6,7,8,9,0};
 cout<<"¿esta vacio?: "<< arreglo.empty()<<endl;
 cout<<"tamaño: "<< arreglo.size()<<endl;
 cout<<"tamaño maximo" << arreglo.max_size()<<endl;
 cout<<"primer elemento: "<<arreglo.front()<<endl;
 cout<<"ultimo elemento: "<<arreglo.back()<<endl;
 cout<<"segundo elemento: "<<arreglo.at(1)<<endl;

array<int,3> a={1,2,3};
array<int,3> b={7,9,9};
a.swap(b);
b.fill(99);
for(int i=0;i<3;i++) {
cout<<a[1]<<endl;
}
for(int i=0;i<3;i++) {
cout<<b[1]<<endl;
}


return 0;
}
