#include "fncalc.hh"
#include "calc_result.hh"

CalcResult::CalcResult(const char *prompt) {
    data = fncalc_process(prompt);
}

CalcResult::~CalcResult() {
    fncalc_free(data);
}

const char *CalcResult::str() {
    return data;
}
