	CPU 8008
    LDI 12
    LEI 8
    CAL &MUL
    CAL &exit
MUL:
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