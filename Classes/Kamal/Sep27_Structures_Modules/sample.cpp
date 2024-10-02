#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <regex>

class Solution {
public:
    // Iterator class for iterating over the parsed integers
    class iterator {
    public:
        // Constructor to initialize the iterator with a vector iterator
        iterator(std::vector<int>::iterator it) : current(it) {}

        // Dereference operator
        int& operator*() {
            return *current;
        }

        // Pre-increment operator
        iterator& operator++() {
            ++current;
            return *this;
        }

        // Post-increment operator
        iterator operator++(int) {
            iterator temp = *this;
            ++current;
            return temp;
        }

        // Comparison operator
        bool operator!=(const iterator& other) const {
            return current != other.current;
        }

    private:
        std::vector<int>::iterator current; // Pointer to the current integer in the vector
    };

    // Constructor that takes an istream reference
    Solution(std::istream &s) {
        std::string line;
        std::regex int_regex(R"(^ *[+-]?\d+ *$)"); // Regex for valid integers
        while (std::getline(s, line)) {
            // Check if the line matches the integer regex
            if (std::regex_match(line, int_regex)) {
                // Convert valid line to integer and add to the vector
                numbers.push_back(std::stoi(line));
            }
        }
    }

    // Function to return the begin iterator
    iterator begin() {
        return iterator(numbers.begin());
    }

    // Function to return the end iterator
    iterator end() {
        return iterator(numbers.end());
    }

private:
    std::vector<int> numbers; // Vector to store valid integers
};

int main() {
    std::istringstream input("#104\n-1\n, -104\n,1\n-1\n+100\n++3\n1.1\nTrues\n1 23\none\n34\n");
    Solution solution(input);

    // Iterate through valid integers and print them
    for (int num : solution) {
        std::cout << num << std::endl; // Output the valid integers
    }

    return 0;
}