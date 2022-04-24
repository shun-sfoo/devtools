#pragma once

typedef enum {
  LEPT_NULL,
  LEPT_TRUE,
  LEPT_FALSE,
} lept_type;

enum {
  LEPT_PARSE_OK = 0,
  LEPT_PARSE_EXPECT_VALUE,
  LEPT_PARSE_INVALID_VALUE,
  LEPT_PARSE_ROOT_NOT_SINGULAR
};

typedef struct {
  lept_type type;
} lept_value;

typedef struct {
  const char* json;
} lept_context;

int lept_parse(lept_value* v, const char* json);

lept_type lept_get_type(const lept_value* v);
