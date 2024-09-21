#include <iostream>

class MyVector {
private:
    int* data;        // Pointer to dynamically allocated array
    std::size_t capacity; // Total allocated memory
    std::size_t size;     // Number of elements in the vector

public:
    // Constructor
    MyVector() : data(nullptr), capacity(0), size(0) {}

    // Destructor to free allocated memory
    ~MyVector() {
        delete[] data;
    }

    // Function to add elements to the vector
    void push_back(int value) {
        // Check if the current capacity is enough to hold a new element
        if (size == capacity) {
            // If not, increase the capacity (double the size or start with 1)
            resize();
        }
        data[size++] = value; // Add the new value and increment the size
    }

    // Function to resize the internal array when necessary
    void resize() {
        capacity = (capacity == 0) ? 1 : capacity * 2;  // Start with capacity 1 or double it
        int* new_data = new int[capacity];              // Allocate new memory

        // Copy old data to the new array
        for (std::size_t i = 0; i < size; ++i) {
            new_data[i] = data[i];
        }

        // Delete the old memory and point data to the new array
        delete[] data;
        data = new_data;
    }

    // Get the number of elements in the vector
    std::size_t get_size() const {
        return size;
    }

    // Access an element at a given index (with bounds checking)
    int& operator[](std::size_t index) {
        if (index >= size) {
            throw std::out_of_range("Index out of range");
        }
        return data[index];
    }

    // Function to print all elements in the vector
    void print() const {
        for (std::size_t i = 0; i < size; ++i) {
            std::cout << data[i] << " ";
        }
        std::cout << std::endl;
    }
};

int main() {
    MyVector vec;

    // Add elements to the vector
    vec.push_back(10);
    vec.push_back(20);
    vec.push_back(30);
    vec.push_back(40);

    // Print the elements in the vector
    vec.print();

    // Access elements
    std::cout << "Element at index 2: " << vec[2] << std::endl;

    // Get vector size
    std::cout << "Size of vector: " << vec.get_size() << std::endl;

    return 0;
}
