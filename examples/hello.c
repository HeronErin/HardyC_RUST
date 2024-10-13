

#include "main.h"

#if CONST_INT == 1
int foo(){
#else
void bar(){
#endif
    int xy = 100;
    int x = ADD_SUB(1, -1) + CONST_INT - CONCAT(x, y);
}

int main()<%
    printf(\"foo\");
%>
