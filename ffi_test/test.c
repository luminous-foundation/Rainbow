#include <windows.h>

int test(char* a, unsigned long long stdOut) {
    return WriteConsoleA(stdOut, a, 1, 0, 0);
}

int test2(int a, int b) {
    return a - b;
}