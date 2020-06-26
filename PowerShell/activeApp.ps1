$y = Get-Process notepad
$y.ID | Sort-Object -Property StartTime -Descending | Out-File activeApp.txt