#ifndef LOGGER_H_
#define LOGGER_H_

#include <stdarg.h>

typedef enum {
	LOG_INFO,
	LOG_WARN,
	LOG_ERROR
} LOG_TYPE;

#if DEBUG
#define log_info(message, ...) log_category(LOG_INFO, __LINE__, __FILE__, message, ##__VA_ARGS__)
#define log_warn(message, ...) log_category(LOG_WARN, __LINE__, __FILE__, message, ##__VA_ARGS__)
#define log_err(message, ...) log_category(LOG_ERROR, __LINE__, __FILE__, message, ##__VA_ARGS__)
#else
#define log_info(message, ...) log_category(LOG_INFO, message, ##__VA_ARGS__)
#define log_warn(message, ...) log_category(LOG_WARN, message, ##__VA_ARGS__)
#define log_err(message, ...) log_category(LOG_ERROR, message, ##__VA_ARGS__)
#endif

#if DEBUG
void log_category(LOG_TYPE type, int line, char* file, const char* message, ...)
	__attribute__((format(printf, 4, 5)));
#else
void log_category(LOG_TYPE type, const char* message, ...)
	__attribute__((format(printf, 2, 3)));
#endif

#endif
