/* Ansi-C style header guard*/
#ifndef MAINH
#define MAINH

#define CONST_INT 5
#define ADD(XXX, YYY) ((XXX) + (YYY))
#define SUB(XXX, YYY) ((XXX) - (YYY)) // bar
#define ADD_SUB(XXXX, YYYY)  ADD(XXXX) - SUB(YYYY) /* foo */ 


// Level of redirection MAKES IT WORK
#define _CONCAT(a, b) a##b
#define CONCAT(a, b) _CONCAT(a, b)


#endif