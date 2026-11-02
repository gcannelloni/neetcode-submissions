#include <string>
#include <vector>

struct Node {
    int data;
    Node* next;
    Node* prev;
};

class LinkedList {
private:
    Node* head;
    Node* tail;

public:
    LinkedList() : head(nullptr), tail(nullptr) {}

    int get(int index) {
        Node* el = head;
        for (int i = 0; i < index; i++) {
            if (el == nullptr) return -1;
            el = el->next;
        }
        if (el == nullptr) return -1;
        return el->data;
    }

    void insertHead(int val) {
        Node* n = new Node{ val, head, nullptr };
        if (head) head->prev = n;
        else tail = n;
        head = n;
    }

    void insertTail(int val) {
        Node* n = new Node{ val, nullptr, tail };
        if (tail) tail->next = n;
        else head = n;
        tail = n;
    }

    bool remove(int index) {
        Node* el = head;
        for (int i = 0; i < index; i++) {
            if (el == nullptr) return false;
            el = el->next;
        }
        if (el == nullptr) return false;

        Node* a = el->prev;
        Node* b = el->next;
        if (a) a->next = b; else head = b;
        if (b) b->prev = a; else tail = a;
        delete el;
        return true;
    }

    std::vector<int> getValues() {
        std::vector<int> elements;
        for (Node* el = head; el != nullptr; el = el->next) {
            elements.push_back(el->data);
        }
        return elements;
    }
};