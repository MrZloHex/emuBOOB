	CPU 8008
init:
	lbi	00001110B
	lci	01110000B
	ldi	0x00

loop:
	lab				; Copy reg b to acc
	orc				; OR reg c into acc
	out 0x00			; acc to out 0 

	cal delay

	lai 0x01			; check direction
	ndd
	
	jtz m_in			; If d==0 move in
	jmp m_out			; Otherwise move out

set_out:
	ldi 0x01			; set direction out
m_out:
	lai	00000001B
	ndb				; Still need to move out?
	jtz	set_in

	lab				; Load acc from b
	rar				; Rotate left
	ori 0x08			; set high nibble bit to 1
	ndi 0x0f			; mask lower nibble
	lba				; save acc back to b

	lac				; Load acc from c
	ral				; Rotate right
	ori 0x10			; set low nibble bit to 1
	ndi 0xf0			; mask lower nibble
	lca				; save acc back to c

	jmp loop			; jump back to loop

set_in:
	ldi 0x00			; set direction in
m_in:
	lai 00001000B
	ndb 				; Still need to move in?
	jtz set_out

	lab				; Load acc from b
	ral				; Rotate left
	ori 0x01			; set low bit to 1
	ndi 0x0f			; mask lower nibble
	lba				; save acc back to b

	lac				; Load acc from c
	rar				; Rotate right
	ori 0x80			; set high bit to 1
	ndi 0xf0			; mask lower nibble
	lca				; save acc back to c

	jmp loop			; jump back to loop

delay:
	lhi 0x1f			; How many times we run iloop
oloop:
	lli 0xff			; Count for inner loop for delay
iloop:
	dcl				; Keep looping until l == 0
	jfz iloop

	dch				; Keep looping until h = 0
	jfz oloop

	ret				; ALl done with our delay

	end
