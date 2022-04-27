# leptjson

## build

1. `cmake -S . -B build`

2. `cmake --build build`

## const

### constant pointers

`int* const ptr`

A constant pointer is a pointer that cannot change the address its holding.

### pointers to constant

`const int* ptr`

These type of pointes can change the address they point to but cannot change the value kept at those address.

### constant pointers to constant

`const int* const ptr` include above;
