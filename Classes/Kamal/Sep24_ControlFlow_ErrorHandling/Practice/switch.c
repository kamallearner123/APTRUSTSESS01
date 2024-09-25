#include <stdio.h>
#include <string.h>

typedef enum COLORS {
    RED,
    GREEN,
    BLUE
} COLORS;

COLORS get_value(char *color) {
    if (strcmp(color, "red") == 0) {
        return RED;
    } else if (strcmp(color, "green") == 0) {
        return GREEN;
    } else {
        return 10;
    }    
}

int main() {
    COLORS result = get_value("bule");
    switch (result) {
        case RED:
            printf("Red returned...");
            break;
        case GREEN:
            printf("Green returne...");
            break;
    }
    printf("Result: %d\n", result);
    return 0;
}
