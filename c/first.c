#include <stdlib.h>

#define MAX_SIZE 100

struct Element {
};

struct Collection {
    struct Element * elements[MAX_SIZE];
    int size;
};

void add_element(struct Collection * self, struct Element * el) {
    if (self->size < MAX_SIZE)
        self->elements[self->size++] = el;
}

void add_collection(struct Collection * self, struct Collection * cl) {
    int i;
    for (i = 0; i < cl->size && self->size < MAX_SIZE; i++)
        self->elements[self->size++] = cl->elements[i];
}

struct Collection a, b;

int main() {
    int i;
    for (i = 0; i < 5; i++) {
        a.elements[i] = (struct Element *) malloc(sizeof(struct Element));
    }
    a.size = i;
    add_element(&a, (struct Element *) malloc(sizeof(struct Element)));
    for (i = 0; i < 4; i++) {
        b.elements[i] = (struct Element *) malloc(sizeof(struct Element));
    }
    b.size = i;
    add_collection(&a, &b);
    return a.size == 10 ? 0 : 1;
}
