# UDP Client

A super simple UDP client example.

## Testing this sample

Set up a UDP listener on port 4444 in order to test this sample. I prefer to use netcat:

```bash
ncat -lvup 4444
```

A quick breakdown of that command: `-l` means listen, `-v` means verbose, `-u` means UDP, `-p 4444` means port 4444. 

**Note:** I am using the updated version of netcat, *ncat*. This should work the same with the old version (*nc*).

After setting up a listener, run the code, and you should see "AAABBBCCC" show up in your netcat listener. Now you can type any message (up to 4096 bytes in length) and hit enter from your netcat window, and you should see the message echoed back into your code terminal.
