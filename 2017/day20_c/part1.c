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

int main() {
    FILE* fin = fopen("input", "r");
    Particle particles[NUM_PARTICLES];
    int x, y, z, ret;
    for (int i = 0; i < NUM_PARTICLES; i++) {
        Particle p;
        ret = fscanf(fin, "p=<%d,%d,%d>, ", &p.p.x, &p.p.y, &p.p.z);
        ret = fscanf(fin, "v=<%d,%d,%d>, ", &p.v.x, &p.v.y, &p.v.z);
        ret = fscanf(fin, "a=<%d,%d,%d>\n", &p.a.x, &p.a.y, &p.a.z);
        particles[i] = p;
    }

    int minA = INT_MAX, minParticle;
    for (int i = 0; i < NUM_PARTICLES; i++) {
        Particle p = particles[i];
        int a = abs(p.a.x) + abs(p.a.y) + abs(p.a.z);
        if (a < minA) {
            minParticle = i;
            minA = a;
        }
    }
    printf("Closest particle: %d", minParticle);
    fclose(fin);

    return 0;
}