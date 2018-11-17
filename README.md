# math
rust implementation for a new primitive type to represent angle radians in another form 
which map from 0 degree (0 radian) ~ 360 degrees (2pi radians) to 0x00* ~ 0xFF* (* depends on number of bits for value)

with more bits to store the value in this type, the more accruate it will be.

this library will also implement some primitive function for calculating the sine and cosine, and basic arithmetic calculation

## Q&A

Why not use the floating point to represent the angle? 

float-point number is kind of bad, it wasted memory space for low precision and it is not normalized even between 2pi and 0; 
