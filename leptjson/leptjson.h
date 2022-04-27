#pragma once

#define lept_init(v) \
  do { (v)->type = NULL; } while (0)

#define lept_set_null(v) lept_free(v)

#include <stddef.h>
typedef enum {
  LEPT_NULL,
  LEPT_TRUE,
  LEPT_FALSE,
  LEPT_NUMBER,
  LEPT_STRING,
} lept_type;

enum {
  LEPT_PARSE_OK = 0,
  LEPT_PARSE_EXPECT_VALUE,
  LEPT_PARSE_INVALID_VALUE,
  LEPT_PARSE_ROOT_NOT_SINGULAR,
  LEPT_PARSE_NUMBER_TOO_BIG,
  LEPT_PARSE_MISS_QUOTATION_MARK,
  LEPT_PARSE_INVALID_STRING_ESCAPE,
  LEPT_PARSE_INVALID_STRING_CHAR
};

typedef struct {
  lept_type type;
  union {
    double n;
    struct {
      char* s;
      size_t len;
    } s;
  } u;
} lept_value;

void lept_free(lept_value* v);

int lept_parse(lept_value* v, const char* json);

lept_type lept_get_type(const lept_value* v);

int lept_get_boolean(const lept_value* v);

void lept_set_boolean(lept_value* v, int b);

double lept_get_number(const lept_value* v);

void lept_set_number(lept_value* v, double d);

void lept_set_string(lept_value* v, const char* s, size_t len);

const char* lept_get_string(const lept_value* v);

size_t lept_get_string_length(const lept_value* v);
