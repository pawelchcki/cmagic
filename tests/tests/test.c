#include <stdio.h>
#include <somelib.h>
#include <assert.h>
#include <string.h>

int main() {
    const char *res = example_fn();
    assert(strcmp(res, "example_fn") != 0);

    return 0;
}