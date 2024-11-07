# Imports

To prevent accidental triggering of aliases, Prosetta requires you to _import_ libraries of aliases. Some aliases are imported by default, while some are parts of a library. Aliases are generally organized by library in these docs, but to be fully sure you should check the documentation's example of each alias to determine which library it is from.

Poems written in Prosetta all have a title and can have any number of authors. The first author can be anything you want, but we recommend writing your own name! The _second_ author and beyond are used for importing libraries. Libraries are [substrings](Glossary.md#substring-matching) within author names. Libraries and examples are shown below.

## Title and Author Formatting
Titles should be the first line of your poem. They are entirely ignored.

The author section starts with the word "by" and then includes any number of authors. The first author is the substring from the word "by" up to the first instance of a comma(,), an ampersand (&), or the word "and", and it is ignored. Beyond the first author, each word you write will be checked for library substrings, and the corresponding libraries will be imported if found.

To end the title/author section, go over two lines.

### Example
This example imports graphics and frames
<editor :code='`
My Poem
by Milo Jacobs, John Graphton, and Alice Framingham.
\t
was frame ide fra 5!
whi mor frame 20!
was frame sub frame 20...
tur ide tim frame frame. 2!
rec tim frame frame!
`' 
:code-wordier=null
output-method='canvas'></editor>

## Graph - Graphics
Any author with the word _graph_ in their name will import the graphics library.

### Example
```
The Ending
by Milo Jacobs and Gilbert Graphton.
```

### Library
The graphics library imports the following aliases:
- [Arc - Circle](Circle.md)
- [Bez - Bezier](Bezier.md)
- [Col - Color](Color.md)
- [Fil - Fill Color](Fill.md)
- [Lin - Line](Line.md)
- [Mov - Move To](MoveTo.md)
- [Pen - Line Width](LineWidth.md)
- [Rec - Rectangle](Rectangle.md)
- [Sto - Stroke Color](Stroke.md)
- [Tur - Rotate](Rotate.md)


## Fram - Animation
Any author with the substring _fram_ in their name will import the graphics library.

Additionally, importing this enables animation as a whole. See [Frame](Frame.md).

### Example
```
The Picture
by Flynn Duniho and Sir Framingham the Mighty.
```

### Library
The graphics library imports the following aliases:
- [fra - Frame](Frame.md)


## List - Lists
Any author with the word _list_ in their name will import the lists library.

### Example
```
The Picture
by Nate Westfall, John Listings, and Spiders Georg.
```

### Library
The graphics library imports the following aliases:
- [fra - Frame](Frame.md)


## Func - Functions
Any author with the substring _func_ in their name will import the functions library.

### Example
```
The Rat Sandwich
by Steve and John Perfunctory.
```

### Library
The functions library imports the following aliases
- [fun - Function](Function.md)
- [ret - Return](Return.md)