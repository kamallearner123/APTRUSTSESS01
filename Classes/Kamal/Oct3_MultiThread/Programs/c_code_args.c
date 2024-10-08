#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

int g_counter;

// Function for the thread to run
void* print_argument(void* arg) {
    char* argument = (char*)arg;
    printf("Thread received argument: %s\n", argument);
    //lock
    g_counter++; // Mistake
    //unlock
    return NULL;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("Usage: %s <arg1> <arg2> ...\n", argv[0]);
        return 1;
    }

    pthread_t* threads = malloc((argc - 1) * sizeof(pthread_t));
    if (threads == NULL) {
        perror("Failed to allocate memory for threads");
        return 1;
    }

    // Create a thread for each command-line argument
    for (int i = 1; i < argc; i++) {
        if (pthread_create(&threads[i - 1], NULL, print_argument, (void*)argv[i]) != 0) {
            perror("Failed to create thread");
            free(threads);
            return 1;
        }
    }

    // Wait for each thread to complete
    for (int i = 0; i < argc - 1; i++) {
        pthread_join(threads[i], NULL);
    }

    // Free the allocated memory
    free(threads);

    return 0;
}

