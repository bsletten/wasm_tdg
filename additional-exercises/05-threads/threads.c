// WebAssembly threads: real pthreads, backed by Web Workers and a
// SharedArrayBuffer for linear memory.
//
// This was the headline "not yet widely supported" feature when the book was
// written. It now works in every major browser -- but only on pages served
// with the two cross-origin isolation headers (see server.py), because
// SharedArrayBuffer is gated behind them.
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <emscripten.h>

#define N_THREADS 4
#define N_ITEMS   (1 << 22)   /* ~4.2M doubles */

static double *data;

typedef struct {
    int id;
    size_t start;
    size_t end;
    double partial;
} work_t;

static void *worker(void *arg) {
    work_t *w = (work_t *)arg;
    double sum = 0.0;
    for (size_t i = w->start; i < w->end; i++) {
        sum += data[i];
    }
    w->partial = sum;
    printf("  thread %d summed [%zu, %zu) -> %.4f\n", w->id, w->start, w->end, sum);
    return NULL;
}

EMSCRIPTEN_KEEPALIVE
double parallel_sum(void) {
    pthread_t threads[N_THREADS];
    work_t work[N_THREADS];
    size_t chunk = N_ITEMS / N_THREADS;

    for (int i = 0; i < N_THREADS; i++) {
        work[i].id = i;
        work[i].start = i * chunk;
        work[i].end = (i == N_THREADS - 1) ? N_ITEMS : (i + 1) * chunk;
        work[i].partial = 0.0;
        pthread_create(&threads[i], NULL, worker, &work[i]);
    }

    double total = 0.0;
    for (int i = 0; i < N_THREADS; i++) {
        pthread_join(threads[i], NULL);
        total += work[i].partial;
    }
    return total;
}

EMSCRIPTEN_KEEPALIVE
double serial_sum(void) {
    double sum = 0.0;
    for (size_t i = 0; i < N_ITEMS; i++) {
        sum += data[i];
    }
    return sum;
}

int main(void) {
    data = malloc(N_ITEMS * sizeof(double));
    for (size_t i = 0; i < N_ITEMS; i++) {
        data[i] = 1.0 / (double)(i + 1);
    }

    printf("Linear memory is a SharedArrayBuffer, so %d threads can read it directly.\n",
           N_THREADS);

    double t0 = emscripten_get_now();
    double serial = serial_sum();
    double t1 = emscripten_get_now();

    printf("\nparallel:\n");
    double t2 = emscripten_get_now();
    double parallel = parallel_sum();
    double t3 = emscripten_get_now();

    printf("\nserial   sum = %.10f  (%.1f ms)\n", serial, t1 - t0);
    printf("parallel sum = %.10f  (%.1f ms)\n", parallel, t3 - t2);
    printf("speedup      = %.2fx\n", (t1 - t0) / (t3 - t2));

    free(data);
    return 0;
}
