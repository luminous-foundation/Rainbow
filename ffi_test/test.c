#include <windows.h>
#include <stdio.h>

int writeConsole(unsigned long long stdOut, char* text, int num) {
    return WriteConsoleA(stdOut, text, num, 0, 0);
}