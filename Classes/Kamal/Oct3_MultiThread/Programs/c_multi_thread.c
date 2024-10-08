#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM_THREADS 4
#define NUM_INCREMENTS 1000000

// Global variable to be incremented by threads
int global_variable = 0;

// Mutex for synchronization
pthread_mutex_t mutex;

void* increment_global(void* arg) {
    for (int i = 0; i < NUM_INCREMENTS; i++) {
        // Lock the mutex to prevent race conditions
        pthread_mutex_lock(&mutex);
        global_variable++;
        // Unlock the mutex after updating the variable
        pthread_mutex_unlock(&mutex);
    }
    return NULL;
}

int main() {
    pthread_t threads[NUM_THREADS];
    int result;

    // Initialize the mutex
    pthread_mutex_init(&mutex, NULL);

    // Create threads
    for (int i = 0; i < NUM_THREADS; i++) {
        result = pthread_create(&threads[i], NULL, increment_global, NULL);
        if (result != 0) {
            printf("Error creating thread %d\n", i);
            exit(EXIT_FAILURE);
        }
    }

    // Wait for all threads to finish
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    // Destroy the mutex
    pthread_mutex_destroy(&mutex);

    // Print the final value of the global variable
    printf("Final value of global variable: %d\n", global_variable);

    return 0;
}

