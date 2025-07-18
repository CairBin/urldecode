#include <stdio.h>
#include <string.h>
#include "./include/urldecode.h"
int main(){
    const char* url = "https%3A%2F%2Fwww.google.ca%2F%3Fgws_rd%3Dssl%23q%3Durl%2Bdecoding";
    char output[1024];
    url_decode_c(url, strlen(url), output, 1024);

    printf("%s\n", output);
}