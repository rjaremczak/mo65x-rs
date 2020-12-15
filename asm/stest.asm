    .org $0600

    ldx #0
    ldy #$ff
loop:
    lda #5
    sta $200,x
    lda #7
    sta $500,y
    inx
    dey
    bne loop
stop:
    jmp stop