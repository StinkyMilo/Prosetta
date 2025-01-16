# Quickstart - Part 2

Next, we will write a basic starter program. Using (the main editor)[https://stinkymilo.github.io/Prosetta/Frontend/], start with the program below.

```
Quickstart Example
by Milo Jacobs and John Graphton


```
## The Code

### The Header

The title and header are written for you. The title is mandatory, but it isn't evaluated. The poem also must have at least one author. The first author is ignored; you may write whatever name you'd like. All other authors are used for *imports*. John Graphton isn't a real person, but writing his name imports the *Graph*ics library.

### Drawing a Circle

On the lines below, write the following code:
```
arc 50. [Draw a circle of radius 50.]
fil red. [Fill the current shape with the color red.]
```

Feel free to skip anything in [square brackets]. These are comments and can be safely ignored.

You should now see a red circle at the center of your screen! If you change the value from 50, you should see different sizes of circle. 

### Adding another circle

We may end up wanting to draw other circles of the same size, but in different locations. Let's add another
circle. Add the following lines:
```
mov 50. [Move 50 pixels]
arc 50.
fil red.
```
You may notice we said to "move 100 pixels" but we didn't specify in what direction. Prosetta uses an invisible "turtle" to draw graphics. This turtle starts in the center of the screen facing right. It can be moved or turned as you see fit, and it is the default drawing location of all graphics.

### Using Variables

Because numbers show up far less often in natural language than they do in code, it's often useful to "hide" numbers when creating a poem around your code. One way to help with that is to use variables to store your numbers. That way, we'll only need to write "50" once rather than three times.

At the beginning of your program, insert this line:
```
was size fifty.
```
Notice that we wrote "fifty" instead of "50". They evaluate to the same thing! More complicated numbers like `one-hundred-seventeen` will also evaluate to `117` as long as they're written with hyphens. 

The `was` alias will assign the next word to the next value after it. In this case, the next word `size` was followed by the value 50, so it creates a variable called `size` with the value `50`. (Special exception: `was` will skip one and two-letter words)

Next, replace every instance of "50" with "size". Your code should now look like this:
```
Quickstart Example
by Milo Jacobs and John Graphton

was size fifty. 
arc size.
fil red.
mov size.
arc size.
fil red.
```

### Using a number constructor.

If we want to hide our one remaining number as well, we can use the "big number constructor", `lit`. (`int` is another way to do this, but unless you want to include the word `pneumonoultramicroscopicsilicovolcanoconiosis-like` in your poem, it's better to use `lit` in this case). 

Replace `fifty` with `lit large conference.` (notice that there should now be 2 periods at the end of the line. We'll discuss punctuation momentarily!) The `lit` alias works by concatenating the *last digit of the lengths* of the words you type after it. `large` has 5 letters, and `conference` has 10. Therefore, we concatenate 5 and 0 and get 50.

Now there aren't any numbers in the poem at all!

### Punctuation

Now's a great time to talk about punctuation! As you may have noticed, Prosetta uses *prefix notation* for everyting. This means function names come before their arguments, including for things like addition (so you say `add 3 2` rather than `3+2`). 

When you've finished supplying all arguments to a function, you must *close* it. You can do this by using punctuation. Each period will close one function, but different punctuation marks will close different numbers of functions at once. Exclamation marks and question marks will close 2, and ellipses will close up to 10. 

For more information on punctuation, see [Punctuation](https://stinkymilo.github.io/Prosetta/Frontend/docs/#/Punctuation)

### Let's Write a Poem!



