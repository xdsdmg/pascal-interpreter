
Program Main;

Procedure Alpha(a : integer; b : integer);

Var x : integer;

Procedure Beta(a : integer; b : integer);

Var x : integer;

Begin
  x := a * 10 + b * 2;
End;

Begin
  x := (a + b ) * 2;
  Beta(5, 10);      { procedure call }
End;

Begin { Main }

  Alpha(3 + 5, 7);  { procedure call }

End.  { Main }
