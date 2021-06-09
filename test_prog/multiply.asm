    ;
    ; multiplying programme
    ;
    ; INTeL I8008
    ;

    CPU 8008

    LDI 8       ; first operand
    LEI 6       ; second operand
    CAL mul     ; calling multiply
    CAL exit    ;
mul:
    LCI 1
    CAL adon    ; adding once
    INC
    LAC
    CPE         ; compare 
    JFZ mul     ; loop 
    RET
adon:
    LAB
    ADD
    LBA
    RET
exit:
    HLT