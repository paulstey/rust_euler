# Problem 111

The current solution (2024-04-06) runs in about 3 minutes on a 48 core node in Oscar. 

After reading the Problem 111 forum on Project Euler, it is clear that that fast solutions aren't creating a list of all 10-digit primes and checking those. Instead, they seem to mostly be generating 10-digit numbers (as strings mostly) with either 9 or 8 repeated values, and then casting them to be integers and checking the primality. 

## To Do
This current solution works, but this problem could use a total re-write that implement the method discussed above.  
