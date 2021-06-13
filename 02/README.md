```
for FILE in $(ls | grep .tst)
do
        echo $FILE:
        hds $FILE
        printf "\n"
done
Add16.tst:
End of script - Comparison ended successfully

ALU-nostat.tst:
End of script - Comparison ended successfully

ALU.tst:
Comparison failure at line 2

FullAdder.tst:
End of script - Comparison ended successfully

HalfAdder.tst:
End of script - Comparison ended successfully

Inc16.tst:
End of script - Comparison ended successfully
```
