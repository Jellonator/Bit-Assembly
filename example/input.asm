; Prompt the user for an input
ext print, "Input something: "
ext prompt

; Get length of input
push 64
ext inputlen, [0:64]

; Allocate memory for input
push [0:64]
ext input, [-[0:64]:[0:64]]

; Echo back user's input
ext print, "You input: "
ext print, [-[0:64]:[0:64]]
ext print, "\n"

; Free memory ( not really necessary here )
pop [0:64]
pop 64
