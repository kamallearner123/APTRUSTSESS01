#include <stdio.h>
#define SIZE 1000000
int main() {
    int mat1[SIZE][SIZE];
    int mat2[SIZE][SIZE];
    int mat3[SIZE][SIZE];
    for (int j=0; j<SIZE; j++) {
        for (int i=0;i<SIZE;i++) {
            mat3[i][j] = mat1[i][j]+mat2[i][j];
        }
    }
}

