"ZORK1 for
	        Zork I: The Great Underground Empire
	(c) Copyright 1983 Infocom, Inc.  All Rights Reserved."

<VERSION ZIP>

<SETG ZORK-NUMBER 1>

<SET REDEFINE T>

<OR <GASSIGNED? ZILCH>
    <SETG WBREAKS <STRING !\" !,WBREAKS>>>

<PRINC "Renovated ZORK I: The Great Underground Empire
">

<FREQUENT-WORDS?>

// See gmacros.zil
<INSERT-FILE "GMACROS" T>
// See gsyntax.zil
<INSERT-FILE "GSYNTAX" T>
// See 1dungeon.zil
<INSERT-FILE "1DUNGEON" T>
// See gglobals.zil
<INSERT-FILE "GGLOBALS" T>

<PROPDEF SIZE 5>
<PROPDEF CAPACITY 0>
<PROPDEF VALUE 0>
<PROPDEF TVALUE 0>

// See gclock.zil
<INSERT-FILE "GCLOCK" T>
// See gmain.zil
<INSERT-FILE "GMAIN" T>
// See gparser.zil
<INSERT-FILE "GPARSER" T>
// See gverbs.zil
<INSERT-FILE "GVERBS" T>
// See 1actions.zil
<INSERT-FILE "1ACTIONS" T>
