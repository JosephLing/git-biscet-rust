### assignment
https://rgrig.github.io/plad/homework.html


### problems:
- not printing out the scores nicely
- remove random picking and use scoring
	- implement binary search method first
	- then the proper counting method?
	- could look into the parents method maybe?


### Fixed:
- integeration test server isn't working
  - can revert if necessary to old version but that can't test multiple instances or repos


### Binary search implementation:
- implement binary search method first
```
// a (good) --> b --> c
//                     \
//                      d (bad)
//                      /
//               f --> e


Originally we were keeping all the good commits as it allowed us to count up from their. However that meant that binary search didn't work. For example we count 6 / 2 = 3 and then if 'e' is a good then we remove 'f' and we keep 'e'. This leads to 5 / 2 = 2.5 = 3 (rounded up).

Therefore if we go for this approach:
- remove all the good commits and unecssary ones (and make sure we don't keep any good ones)

// 			    b --> c
//                     \
//                      d (bad)
//                      /
//               f --> e

5 / 2 = 2.5 = 3 which gives 'e'

// 			    b --> c
//                     \
//                      d (bad)
//                      /
//               f --> e


3 / 2 = 1.5 = 2 which gives 'c'
// 			    b --> c
//                     \
//                      d (bad)

```
Although it doesn't seem to be working properly yet :(