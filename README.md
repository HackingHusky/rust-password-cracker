# rust-password-cracker

Please use with cation and use with promission

How to Use:
Save the script to a file, for example, password_cracker.rs.

Create a wordlist file named wordlist.txt in the same directory as the script. This file should contain potential passwords, one per line.

Compile the script using Rust's compiler:

bash
rustc password_cracker.rs
Run the script with the password hash as an argument:

bash
./password_cracker <password_hash>
Replace <password_hash> with the actual hash you want to crack. The script will read through the wordlist, hash each word using SHA-256, and compare it to the provided hash. If a match is found, it will print the password and the time taken to find it.

This is coded for sha256. 

Remember to use this script responsibly and ethically! 
