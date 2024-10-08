#include <stdio.h>

int g_conns; // data seg

int add(int a, int b) {
    return a+b;
}


int main() {
    int a = 10; // Stack
    int r = add(a,10); // stack
    printf(" r = %d\n", r);

    if (r>100) {
        printf("fdsafsfds");
    } else if (r < 0){
        ///
    } else {
    }

    return 0;
}
