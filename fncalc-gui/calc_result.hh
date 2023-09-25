#pragma once

class CalcResult {
    private:
        char *data;
    public:
        CalcResult(const char *prompt);
        ~CalcResult();

        const char *str();
};
