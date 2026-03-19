#include "logger.h"

#include <stdio.h>

#define COLOR_RESET "\033[0m"
#define COLOR_INFO  "\033[1;37m"
#define COLOR_WARN  "\033[1;33m"
#define COLOR_ERROR "\033[1;31m"

#if DEBUG
void log_category(LOG_TYPE type, int line, char* file, const char* message, ...) {
#else
void log_category(LOG_TYPE type, const char* message, ...) {
#endif
	va_list args;
	va_start(args, message);

	switch (type) {
		case LOG_WARN:
            fputs(COLOR_WARN"WARN"COLOR_RESET"    ", stderr);
            break;
		case LOG_ERROR:
            fputs(COLOR_ERROR"ERROR"COLOR_RESET"   ", stderr);
            break;
		case LOG_INFO:
            fputs(COLOR_INFO"INFO"COLOR_RESET"    ", stderr);
            break;
		default:
			break;
	}
    vfprintf(stderr, message, args);
#if DEBUG
    if (type != LOG_INFO) {
        fprintf(stderr, " (%s:%d)", file, line);
    }
#endif
    fputs("\n", stderr);

	va_end(args);
}
