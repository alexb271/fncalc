#ifndef FNCALC_FFI
#define FNCALC_FFI

extern "C" char *fncalc_process(const char *);
extern "C" char *fncalc_reset();
extern "C" void fncalc_free(char *);

#endif
