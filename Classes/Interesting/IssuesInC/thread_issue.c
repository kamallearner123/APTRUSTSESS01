#include <stdio.h>
#include <pthread.h>

int shared_counter = 0;  // Global shared variable

void* increment_counter(void* arg) {
    // for (int i = 0; i < 1000000; i++) {
    //     shared_counter++;  // Data race here
    // }
    return NULL;
}

int main() {
    pthread_t thread1, thread2;

    // Create two threads that increment the shared_counter
    pthread_create(&thread1, NULL, increment_counter, NULL);
    pthread_create(&thread2, NULL, increment_counter, NULL);

    // Wait for both threads to complete
    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);

    // Print the final value of shared_counter
    printf("Final counter value: %d\n", shared_counter);

    return 0;
}
