import {EditorView, minimalSetup} from "codemirror"
import {javascript} from "@codemirror/lang-javascript"

let editor = new EditorView({
  extensions: [minimalSetup, javascript()],
  parent: document.getElementById("code")
})
