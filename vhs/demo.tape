Output vhs/demo.gif

Require wordperms

Set Shell zsh
Set Theme "tokyonight-storm"
Set Width 1800
Set Height 1000
Set Padding 20

Set TypingSpeed 100ms
Type 'echo -e "apple\nbanana\n123" > wordlist.txt'
Enter
Type "cat wordlist.txt"
Enter
Sleep 500ms
Type "wordperms -i wordlist.txt"
Enter
Sleep 5s
Hide
Type@1ms "rm wordlist.txt"
Enter
