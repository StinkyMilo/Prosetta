const express = require('express');
const app = express();

app.use(express.static('./'));
app.listen(8080, () => {
  console.log("Listening on port 8080, http://localhost:8080");
});
