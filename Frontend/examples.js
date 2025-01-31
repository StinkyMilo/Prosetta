import { updateValue } from './script.js';
const examples = {
  blank: `TITLE
by YOUR NAME and IMPORTS

CODE HERE
`,
  summerRose: `The Wreck of the Summer Rose
by Milo Jacobs, Steve Graphton, and Lord Framingham.

The waves rose ten yards out from the sea.
The storm of the dark red moon had taken our best men.
The open waters pulled them into the depths!
The water whipped at the boat,
    releasing its rose-tinted wood three-hundred fathoms below.
The wreck of the Summer Rose left few survivors.
    Nature's tide had taken all our formidable men and left only 2.
    The storm gave no respite.
    As the waters rose, those who were left abandoned the 
    Summer Rose and swam 10 hours before finding rescue!
It is truly a miracle that they lived.`,
  paperIntro: `Intro to Prosetta
by Milo Jacobs, Flynn Duniho, Nate Westfall, Steve Graphton, The Trigonometry Baby, and Lord Framingham

Prosetta is an esoteric programming language where code is a "hidden message" within a poem or other piece of writing. Its primary focus is on graphical output.

Whereas many Esolangs have very light language capabilities --- often intentionally minimizing usability --- a goal of Prosetta is to provide ample language features to enable user creativity.

One of the ways we do this is through an online editor that gives the user timely information and pointers about how they can best hide their code within poems as well as syntax highlighting...

A major motivation for the use of Prosetta is the pastime of the constrained writing challenge. This type of challenge can involve omitting a specific letter, requiring the use of many pre-determined words, or hiding a secret word in the first letter of each sentence. 
Prosetta allows the creation of multimedia art involving code, writing, and visuals. We hope that by using this language, many people will be able to create art that is at once code, poetry, and a digital drawing or animation!

To achieve the goal of a hidden message, most characters of Prosetta code are quite literally simply ignored. Prosetta has three-letter 'aliases' to represent each of the language's capabilities and functions!

The placement of the three letters does not have to be contiguous, but the characters must be in the right order (i.e. they can be split or not.). Prosetta is like lisp in that aliases act as prefix-notated function calls. 

Prosetta code tends to be quite well hidden within a larger text. For example, this intro was and still is a program that draws a light green, rotating eye.

The story and output of Prosetta code can be poetically related. Alternatively, they can be firmly unrelated. A user can juxtapose text and output to varying effects: output can subvert, emphasize, or add entirely new meaning to the poem itself. 

This research paper will highlight development methods: the way we made the program, as well as rationale: why we made the decisions we did and what purpose they served. 

We will start with design, including a background literature review and our design goals.

The next part will cover the ways the language was implemented and our reasons for those choices. This will include technical setup, features, documentation, and revision.

Lastly, we will cover the assessment of the language: how we determined whether we had fulfilled the goals of the language as well as how others have used it.
`

}
function updateExample() {
  let poem = examples[document.getElementById("poemSelect").value];
  updateValue(poem);
}
export default updateExample;
