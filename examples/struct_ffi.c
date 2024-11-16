#include <stdio.h>

typedef struct {
    int a;
    float b;
} Foo;

void bar(Foo f) {
    printf("%s\n", "this is C code, printing out a struct passed from Rainbow");
    printf("%d\n", f.a);
    printf("%f\n", f.b);
}

Foo baz(int a, float b) {
    Foo f;
    f.a = a;
    f.b = b;

    return f;
}
