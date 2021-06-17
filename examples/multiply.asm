	CPU 8008
    LDI 101b
    LEI Ah
    CAL &mul
    CAL &exit
mul:
    LCI 0
lood:
    CAL &adon
    INC
    LAC
    CPE
    JFZ &lood
    RET
adon:
    LAB
    ADD
    LBA
    RET
exit:
    HLT