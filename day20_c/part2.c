#include "stdlib.h"
#include "stdio.h"
#include "limits.h"

#define NUM_PARTICLES 1000

typedef struct Tuple {
    int x;
    int y;
    int z;
} Tuple;

typedef struct Particle {
    Tuple p;
    Tuple v;
    Tuple a;
} Particle;

typedef struct Node {
    Particle *data;
    struct Node *next;
    struct Node *prev;
} Node;

void tick(Node *particles) {
    Node *n = particles;
    while (n != NULL) {
        Particle *p = n->data;
        p->v.x += p->a.x;
        p->v.y += p->a.y;
        p->v.z += p->a.z;

        p->p.x += p->v.x;
        p->p.y += p->v.y;
        p->p.z += p->v.z;

        n = n->next;
    }
}

int count(Node *ns) {
    Node *n = ns;
    int c;
    for (c = 0; n != NULL; c++, n = n->next);
    return c;
}

Node* detectCollisions(Node *particles, int* deleted) {
    Node *toDelete[NUM_PARTICLES] = {0};
    int d = 0;

    Node *p = particles, *head = particles, *prev = NULL, *q;
    int shouldDelete;
    *deleted = 0;
    while (p != NULL) {
        shouldDelete = 0;
        q = p->next;
        while (q != NULL) {
            Tuple pos1 = p->data->p, pos2 = q->data->p;
            if (pos1.x == pos2.x && pos1.y == pos2.y && pos1.z == pos2.z) {
                toDelete[d++] = q;
                *deleted = 1;
            }
            q = q->next;
        }

        if (d > 0) {
            toDelete[d++] = p;
            while (d > 0) {
                d--;
                Node *n = toDelete[d];
                if (n == head) {
                    head = n->next;
                }
                if (p == n) {
                    p = n->next;
                }
                if (n->prev != NULL) {
                    n->prev->next = n->next;
                }
                if (n->next != NULL) {
                    n->next->prev = n->prev;
                }
                free(n->data);
                free(n);
            }
        } else {
            p = p->next;
        }
    }
    return head;
}

int main() {
    FILE *fin = fopen("input", "r");
    Node *p, *next = NULL;
    int x, y, z;
    for (int i = 0; i < NUM_PARTICLES; i++) {
        Particle *new = malloc(sizeof(Particle));
        fscanf(fin, "p=<%d,%d,%d>, ", &new->p.x, &new->p.y, &new->p.z);
        fscanf(fin, "v=<%d,%d,%d>, ", &new->v.x, &new->v.y, &new->v.z);
        fscanf(fin, "a=<%d,%d,%d>\n", &new->a.x, &new->a.y, &new->a.z);

        p = malloc(sizeof(Node));
        p->data = new;
        p->next = next;
        if (next != NULL) {
            next->prev = p;
        }
        p->prev = NULL;
        next = p;
    }
    Node *particles = p;

    int deleted, withoutDelete = 0;
    while (withoutDelete < 100) {
        tick(particles);
        particles = detectCollisions(particles, &deleted);
        if (!deleted) {
            withoutDelete++;
        }
    }
    printf("Remaining particles: %d", count(particles));
    fclose(fin);

    return 0;
}