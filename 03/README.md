```
→ cd a
→ for FILE in $(ls | grep .tst)
do
        echo $FILE:
        hds $FILE
        printf "\n"
done
Bit.tst:
End of script - Comparison ended successfully

PC.tst:
End of script - Comparison ended successfully

RAM64.tst:
End of script - Comparison ended successfully

RAM8.tst:
End of script - Comparison ended successfully

Register.tst:
End of script - Comparison ended successfully

→ cd ../b
→ for FILE in $(ls | grep .tst)
do
        echo $FILE:
        hds $FILE
        printf "\n"
done
RAM16K.tst:
End of script - Comparison ended successfully

RAM4K.tst:
End of script - Comparison ended successfully

RAM512.tst:
End of script - Comparison ended successfully

```
