#include <iostream>
#include <memory>

class Node {
public:
    std::shared_ptr<Node> next;
    ~Node() {
        std::cout << "Node destroyed\n";
    }
};

int main() {
    std::shared_ptr<Node> node1 = std::make_shared<Node>();
    std::shared_ptr<Node> node2 = std::make_shared<Node>();

    // Create a cyclic reference
    node1->next = node2;
    node2->next = node1;

    // Neither node1 nor node2 will be destroyed
    // because they are in a reference cycle.
    std::cout << "End of main\n";
}
