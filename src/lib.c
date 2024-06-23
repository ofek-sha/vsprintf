#include <stdio.h>
#include <stdarg.h>

int vsnprintf_wrapper(char *buffer,
                      size_t size,
                      const char *format,
                      va_list orig_list) {
    va_list list;
    va_copy(list, orig_list);
    return vsnprintf(buffer, size, format, list);
}

