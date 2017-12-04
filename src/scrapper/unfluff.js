const extractor = require("unfluff");
const axios = require("axios");

axios.get(process.argv[2], { timeout: 5000 }).then(res => {
  const { text, softTitle } = extractor(res.data);
  console.log(JSON.stringify({ content: text, title: softTitle }));
});
