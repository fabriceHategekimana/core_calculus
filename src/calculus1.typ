= stlc

== Syntax
t ::= term

	$lambda x. y$		type abstraction



== Evaluation
	$ ("t1" -> "tp1"; )/("t1" "t2" -> "tp1" "t2")  & "APP1" $
	$ ("t1" -> "tp1"; )/("t1" "t2" -> "tp1" "t2")  & "APP2" $
	$ ("t1" -> "tp1"; )/("t1" "t2" -> "tp1" "t2")  & "APP3" $

== Typing
	$ ("t1" : "T1" -> "T2"; "t2": "T1"; )/("t1" "t2" : "T1")  & "TYP1" $
	$ ("t1" : "T1" -> "T2"; "t2": "T1"; )/("t1" "t2" : "T1")  & "TYP2" $

