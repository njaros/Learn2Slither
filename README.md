## Doc ##

- https://fr.wikipedia.org/wiki/Q-learning#:~:text=En%20intelligence%20artificielle%2C%20plus%20pr%C3%A9cis%C3%A9ment,un%20%C3%A9tat%20donn%C3%A9%20du%20syst%C3%A8me.


this wiki page is surprisely complete

## Algorythm ##

- BellMan equation

## Steps ##

Per episode, do:  
    - a serie of action depending of the states thanks of the Q table (or the random action)  
    - update the q table  


At end:  
    - save model (q table)  

## To do ##

Define what an episode is in our game:  
    - 1 episode = 1 state ?  
    - 1 episode = n states ?  
    - 1 episode = until death ?  


Define hot to modify q table:  
    - Monte Carlo ?  
    - "TD" ?  


When to modify the Q table:  
    - at end of each episodes ?  
    - every n steps ? <-- seems more appropriate

## States ##

### 1- simple impl: lowcost + just on side awareness ###

cardinal product of just beside Right->[E, S, #, G, R] * Up->[...] * Down[...] * Left->[...] = 5 * 5 * 5 * 5 = 625 different states

### 2- more complex impl: cost too high + distance awareness ###

let # = distance of wall
'#' = {1, 2, 3, 4+}

let S = number of S in this side
S = {0, 1, 2, 3+}

let G = distance of first green apple if no S between
G = {1, 2, 3, 4+}

let R = distance of Red apple
R = {1, 2, 3, 4+}

let B = what there is directly beside
B = {E, S, #, G, R}

cardinal product for each side: (card(#) * card(S) * card(G) * card(R) * card(B)) ^ card(sides) = 2.68*e12  
cardinal too big

### 3- cost acceptable + self lenght awareness + blind on close sides ###

if we get just another parameter than B which are 3 possibilities,
state cardinal = 50625, going further is probably a bad idea.

adjust:  
let N = what is shorter between {S, #, G, R} = 4  
let S = number of S on side {0, 1, 2+} = 3

total cardinal = (card(N)*card(S))^(sides) = 20736 -> more acceptable

### 4- another easy impl: very lowcost + stupid ###

let N = what is shorter between {S or #, G, R} = 3  
total cardinal = 3^4 = 81

### 5- another impl: lowcost + Dead Near detection ###

let N = what is shorter between {S or #, G, R} = 3  
let D = is S or # just beside {0, 1} = 2  
total cardinal = (card(N) * card(G)) ^ card(sides) = 576

### balance sheet ###

impl 5 seems to be a good first impl to try.


## Rewards ##

first think:  
    Ate green -> +10  
    Ate red -> -10  
    Deadge -> -1000  
    Empty tile -> -1  
