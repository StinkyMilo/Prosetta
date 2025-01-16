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
You may notice we said to "move 50 pixels" but we didn't specify in what direction. Prosetta uses an invisible "turtle" to draw graphics. This turtle starts in the center of the screen facing right. It can be moved or turned as you see fit, and it is the default drawing location of all graphics.

### Using Variables

Because numbers show up far less often in natural language than they do in code, it's often useful to "hide" numbers when creating a poem around your code. One way to help with that is to use variables to store your numbers. That way, we'll only need to write "50" once rather than three times.

At the beginning of your program, insert this line:
```
was inside fifty.
```
Notice that we wrote "fifty" instead of "50". They evaluate to the same thing! More complicated numbers like `one-hundred-seventeen` will also evaluate to `117` as long as they're written with hyphens. 

The `was` alias will assign the next word to the next value after it. In this case, the next word `inside` was followed by the value 50, so it creates a variable called `inside` with the value `50`. (Special exception: `was` will skip one and two-letter words)

Next, replace every instance of "50" with "inside". Your code should now look like this:
```
Quickstart Example
by Milo Jacobs and John Graphton

was inside fifty. 
arc inside.
fil red.
mov inside.
arc inside.
fil red.
```

### Using a number constructor.

If we want to hide our one remaining number as well, we can use the "big number constructor", `lit`. (`int` is another way to do this for small numbers, but it's better to use `lit` in this case). 

Replace `fifty` with `lit demon conference.` (notice that there should now be 2 periods at the end of the line. We'll discuss punctuation momentarily!) The `lit` alias works by concatenating the *last digit of the lengths* of the words you type after it. `demon` has 5 letters, and `conference` has 10. Therefore, we concatenate 5 and 0 and get 50.

Now there aren't any numbers in the poem at all!

### Punctuation

Now's a great time to talk about punctuation! As you may have noticed, Prosetta uses *prefix notation* for everyting. This means function names come before their arguments, including for things like addition (so you say `add 3 2` rather than `3+2`). 

When you've finished supplying all arguments to a function, you must *close* it. You can do this by using punctuation. Each period will close one function, but different punctuation marks will close different numbers of functions at once. Exclamation marks and question marks will close 2, and ellipses will close up to 10. 

With this info, let's try replacing the ugly `..` at the end of line 4 with `!`.

For more information on punctuation, see [Punctuation](Punctuation.md)

### Let's Write a Poem! 

Let's take a look at our new code below. This, of course, doesn't really look like a poem yet, but that can be changed! Try hovering your mouse over `fil` and clicking `familiar`. Then we can turn line 6 into a sentence. Maybe, `The sky glowed a familiar red.` Notice all the new letters we added are not highlighted. This means the compiler ignores them entirely.

You can continue this process for as long as you want. Try replacing other aliases and adding in other words to make your poem into complete sentences. You can also change the word `inside` to whatever else you want. Lastly, if you try hovering over `demon` or `conference`, you'll see suggestions for other words of the same length. 

If you get stuck in this process and want ideas, feel free to view the `wordier` version of the code below.

<editor :code="`
Quickstart Example
by Milo Jacobs and John Graphton\n
was inside lit demon conference!
arc inside.
fil red.
mov inside.
arc inside.
fil red.
`" 
:code-wordier="`
Evil Demon Time
by Milo Jacobs and John Graphton\n
There I was, inside the Elite Demon Conference!
I needed to sacrifice a goat to get inside.
The sky glowed a familiar red.
As I moved inside, I saw the demons in their natural habitat.
They searched inside the building for anyone out of place.
The filthy floor was red with the blood of those they found.
`"
output-method='canvas'></editor>

## Dealing with Errors
You'll likely run into problems using Prosetta. Unlike other languages, Prosetta will never throw an error. Instead, it will simply ignore any part of the code it doesn't understand. This can make debugging difficult when compared to other languages. Our main recommendation is to pay attention to syntax highlighting and JavaScript output. When code isn't giving you the output you expect, check what parts are being ignored. Common errors include using the wrong number of arguments and not closing a function with the proper punctuation.

## Using the Rest of the Docs

Hopefully, this Quickstart helped you learn the basics of Prosetta. But there are, of course, many features you haven't used yet! Feel free to browse through the rest of the docs to see both a minimal and wordier example of almost every alias. 

From here, try whatever you want! We recommend doing a similar process as we used in this Quickstart, wherein you first decide what to draw and then turn the code into a poem or piece of prose.
