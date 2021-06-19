we skip Memory.tst in this output because it requires manual input that we only
get with the hardware simulator.
```
→ for FILE in $(ls | grep .tst | grep -v Memory.tst)
do
        echo $FILE:
        hds $FILE
        printf "\n"
done
ComputerAdd-external.tst:
End of script - Comparison ended successfully

ComputerAdd.tst:
End of script - Comparison ended successfully

ComputerMax-external.tst:
End of script - Comparison ended successfully

ComputerMax.tst:
End of script - Comparison ended successfully

ComputerRect-external.tst:
End of script - Comparison ended successfully

ComputerRect.tst:
End of script - Comparison ended successfully

CPU-external.tst:
End of script - Comparison ended successfully

CPU.tst:
End of script - Comparison ended successfully
```
