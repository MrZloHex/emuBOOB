	CPU 8008
    LDI 12
    LEI 8
    CAL &mul
    CAL &exit
mul:
    LCI 0
loop:
    CAL &adon
    INC
    LAC
    CPE
    JFZ &loop
    RET
adon:
    LAB
    ADD
    LBA
    RET
exit:
    HLT