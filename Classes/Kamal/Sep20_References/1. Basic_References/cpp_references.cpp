// #include <iostream>

// // void feed_seed(int &num) {
// //     num +=10;
// // }


// void feed_seed(int num) {
//     int result = num +=10;
// }

// int main() {
//     int a = 10;
//     int &b = a; // Caution: Purpose is not clear.
//     b = 200; // Updating
//     std::cout << a << "," << b << std::endl;

//     feed_seed(a); // Caution: No trust on third party lib.
//     std::cout << a << "," << b << std::endl;
// }



#include <iostream>

// void feed_seed(int &num) {
//     num +=10;
// }


int& fun() {
    int a = 10;
    return a;
}

int main() {
    int &ref = fun();
    // UNDEFINED
    std::cout << ref << std::endl;
}