# Numbers

Integer literals can be constructed in a few different ways. To hide them among the text, see the int and lit aliases. To write them directly, you can type the number as you would in most programming languages (e.g. `0`, `741`, `-3`). You can also write them as hyphen-separated english-language descriptions of the language. This includes things like `one` (`1`), `fifteen` (`15`), `sixty-seven` (`67`), `one-hundred-forty-three-million-two-hundred-and-five` (`143,000,205`), and some other weird items like `two-dozen` (`24`) and `four-score-and-seven` (`87`). This all works as long as the number names are separated by hyphens, not spaces or anything else.

# Colors

Colors can be constructed with the color constructor, but their names can be directly stated. This works for any color recognized as a string in JavaScript/CSS, a list of which can be found [here](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color). For colors that are multiple words, you can break them up at the word level. For example, `darkslategrey`, `dark slategrey`, `darkslate grey`, and `dark slate grey` will all evaluate to `darkslategrey`. These color literals are stored as strings in JS and can be passed into any function that takes a color.

# Strings

String literals are constructed as they are in most languages, by putting words between double quotes ("). In contrast to other languages, however, single quotes are ignored entirely (therefore, use single quotes if you want to have dialogue that still triggers aliases). String literals are also automatically formatted. If a variable name is detected within the string, the string will be automatically formatted with that variable's value. There is no way to avoid this, so name your variables carefully!